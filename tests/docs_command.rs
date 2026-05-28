use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::{Command, Output};
use std::sync::{Arc, Mutex};
use std::thread;

fn jbx_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_jbx"))
}

fn assert_success(out: &Output) {
    assert!(
        out.status.success(),
        "expected success\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn serve_files(files: HashMap<&'static str, Vec<u8>>) -> (String, Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&requests);
    thread::spawn(move || loop {
        let Ok((mut stream, _)) = listener.accept() else {
            break;
        };
        let mut request = [0_u8; 2048];
        let read = stream.read(&mut request).unwrap_or(0);
        let request_text = String::from_utf8_lossy(&request[..read]);
        let path = request_text
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/")
            .to_string();
        seen.lock().unwrap().push(path.clone());
        let (status, body): (&str, &[u8]) = match files.get(path.as_str()) {
            Some(body) => ("200 OK", body.as_slice()),
            None => ("404 Not Found", b"not found"),
        };
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(body).unwrap();
    });
    (base, requests)
}

#[test]
fn docs_local_source_defaults_to_markdown_and_does_not_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let script = tmp.path().join("Hello.java");
    fs::write(
        &script,
        r#"//DESCRIPTION Says hello to agents
//DOCS usage=Run it with a name.
//DEPS info.picocli:picocli:4.7.7
void main() {
  IO.println("hello");
}
"#,
    )
    .unwrap();
    let cache = tmp.path().join("cache");

    let out = jbx_command()
        .arg("docs")
        .arg(&script)
        .arg("--cache-dir")
        .arg(&cache)
        .output()
        .unwrap();

    assert_success(&out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("# Hello.java"), "{stdout}");
    assert!(stdout.contains("Says hello to agents"), "{stdout}");
    assert!(stdout.contains("usage: Run it with a name."), "{stdout}");
    assert!(stdout.contains("info.picocli:picocli:4.7.7"), "{stdout}");
    assert!(
        !cache.exists(),
        "local docs generation should not create a cache directory"
    );
}

#[test]
fn docs_local_source_json_includes_structured_types_and_members() {
    let tmp = tempfile::tempdir().unwrap();
    let source = tmp.path().join("Widget.java");
    fs::write(
        &source,
        r#"package dev.telegraphic.demo;

@Deprecated(since = "1.0", forRemoval = false)
public class Widget extends BaseWidget implements java.io.Serializable {
  public static final String KIND = "widget";
  private int count;

  public Widget(String name) {}

  @Deprecated
  public java.util.List<String> names(String prefix, int limit) throws java.io.IOException {
    return java.util.List.of(prefix);
  }

  protected int size() { return count; }
}
"#,
    )
    .unwrap();

    let out = jbx_command()
        .arg("docs")
        .arg(&source)
        .arg("--json")
        .output()
        .unwrap();

    assert_success(&out);
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let ty = &json["types"][0];
    assert_eq!(ty["kind"], "class");
    assert_eq!(ty["name"], "Widget");
    assert_eq!(ty["qualifiedName"], "dev.telegraphic.demo.Widget");
    assert_eq!(ty["package"], "dev.telegraphic.demo");
    assert_eq!(ty["visibility"], "public");
    assert_eq!(
        ty["annotations"][0]["qualifiedName"],
        "java.lang.Deprecated"
    );
    assert_eq!(ty["extends"], "dev.telegraphic.demo.BaseWidget");
    assert_eq!(ty["implements"][0], "java.io.Serializable");
    assert_eq!(
        ty["fields"][0]["qualifiedName"],
        "dev.telegraphic.demo.Widget.KIND"
    );
    assert_eq!(ty["fields"][0]["type"], "String");
    assert_eq!(ty["fields"][0]["visibility"], "public");
    assert_eq!(ty["constructors"][0]["parameters"][0]["type"], "String");
    let method = ty["methods"]
        .as_array()
        .unwrap()
        .iter()
        .find(|method| method["name"] == "names")
        .unwrap();
    assert_eq!(method["qualifiedName"], "dev.telegraphic.demo.Widget.names");
    assert_eq!(method["visibility"], "public");
    assert_eq!(method["returnType"], "java.util.List<String>");
    assert_eq!(method["parameters"][0]["name"], "prefix");
    assert_eq!(method["parameters"][0]["type"], "String");
    assert_eq!(method["parameters"][1]["type"], "int");
    assert_eq!(method["throws"][0], "java.io.IOException");
    assert_eq!(
        method["annotations"][0]["qualifiedName"],
        "java.lang.Deprecated"
    );
}

#[test]
fn docs_local_jar_json_includes_structured_types_from_javap() {
    let tmp = tempfile::tempdir().unwrap();
    let source_dir = tmp.path().join("src/dev/telegraphic/demo");
    fs::create_dir_all(&source_dir).unwrap();
    let source = source_dir.join("JarWidget.java");
    fs::write(
        &source,
        r#"package dev.telegraphic.demo;

public class JarWidget {
  public static final String KIND = "jar";
  protected int count;
  public JarWidget() {}
  public String greet(String name) { return name; }
  protected int size() { return count; }
}
"#,
    )
    .unwrap();
    let classes = tmp.path().join("classes");
    fs::create_dir_all(&classes).unwrap();
    let javac = Command::new("javac")
        .arg("-parameters")
        .arg("-d")
        .arg(&classes)
        .arg(&source)
        .output()
        .unwrap();
    assert_success(&javac);
    let jar = tmp.path().join("widgets.jar");
    let jar_out = Command::new("jar")
        .arg("--create")
        .arg("--file")
        .arg(&jar)
        .arg("-C")
        .arg(&classes)
        .arg(".")
        .output()
        .unwrap();
    assert_success(&jar_out);

    let out = jbx_command()
        .arg("docs")
        .arg(&jar)
        .arg("--json")
        .output()
        .unwrap();

    assert_success(&out);
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let ty = &json["types"][0];
    assert_eq!(ty["qualifiedName"], "dev.telegraphic.demo.JarWidget");
    assert_eq!(ty["fields"][0]["name"], "KIND");
    let greet = ty["methods"]
        .as_array()
        .unwrap()
        .iter()
        .find(|method| method["name"] == "greet")
        .unwrap();
    assert_eq!(greet["returnType"], "String");
    assert_eq!(greet["parameters"][0]["name"], "name");
    assert_eq!(greet["parameters"][0]["type"], "String");
    let size = ty["methods"]
        .as_array()
        .unwrap()
        .iter()
        .find(|method| method["name"] == "size")
        .unwrap();
    assert_eq!(size["visibility"], "protected");
}

#[test]
fn docs_remote_gav_fetches_markdown_sidecar_and_reuses_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let docs = b"# Remote docs\n\nUse this from cache.\n".to_vec();
    let (repo, requests) = serve_files(HashMap::from([(
        "/dev/telegraphic/demo/1.0.0/demo-1.0.0-jbx-docs.md",
        docs,
    )]));
    let cache = tmp.path().join("cache");

    for _ in 0..2 {
        let out = jbx_command()
            .arg("docs")
            .arg("dev.telegraphic:demo:1.0.0")
            .arg("--repo")
            .arg(format!("local={repo}"))
            .arg("--cache-dir")
            .arg(&cache)
            .output()
            .unwrap();
        assert_success(&out);
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            "# Remote docs\n\nUse this from cache.\n"
        );
    }

    let seen = requests.lock().unwrap();
    assert_eq!(
        seen.iter()
            .filter(|path| path.as_str() == "/dev/telegraphic/demo/1.0.0/demo-1.0.0-jbx-docs.md")
            .count(),
        1,
        "remote sidecar should be fetched once then served from cache: {seen:?}"
    );
    assert!(
        cache
            .join("docs/dev/telegraphic/demo/1.0.0/demo-1.0.0-jbx-docs.md")
            .exists(),
        "expected cached docs sidecar under docs namespace"
    );
}

#[test]
fn docs_remote_group_artifact_resolves_latest_docs_sidecar() {
    let tmp = tempfile::tempdir().unwrap();
    let (repo, requests) = serve_files(HashMap::from([
        (
            "/com/fasterxml/jackson/core/jackson-databind/maven-metadata.xml",
            br#"<metadata>
  <groupId>com.fasterxml.jackson.core</groupId>
  <artifactId>jackson-databind</artifactId>
  <versioning>
    <release>2.17.2</release>
    <versions>
      <version>2.17.1</version>
      <version>2.17.2</version>
    </versions>
  </versioning>
</metadata>"#.to_vec(),
        ),
        (
            "/com/fasterxml/jackson/core/jackson-databind/2.17.2/jackson-databind-2.17.2-jbx-docs.md",
            b"# Jackson Databind docs\n\nResolved latest sidecar.\n".to_vec(),
        ),
    ]));

    let out = jbx_command()
        .arg("docs")
        .arg("com.fasterxml.jackson.core:jackson-databind")
        .arg("--repo")
        .arg(format!("local={repo}"))
        .arg("--cache-dir")
        .arg(tmp.path().join("cache"))
        .output()
        .unwrap();

    assert_success(&out);
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "# Jackson Databind docs\n\nResolved latest sidecar.\n"
    );
    let seen = requests.lock().unwrap();
    assert!(
        seen.iter().any(|path| path.as_str()
            == "/com/fasterxml/jackson/core/jackson-databind/maven-metadata.xml"),
        "expected latest-version metadata lookup: {seen:?}"
    );
    assert!(
        seen.iter().any(|path| path.as_str()
            == "/com/fasterxml/jackson/core/jackson-databind/2.17.2/jackson-databind-2.17.2-jbx-docs.md"),
        "expected resolved latest docs sidecar fetch: {seen:?}"
    );
}

#[test]
fn docs_remote_gav_json_fetches_json_sidecar() {
    let tmp = tempfile::tempdir().unwrap();
    let (repo, _) = serve_files(HashMap::from([(
        "/dev/telegraphic/demo/1.0.0/demo-1.0.0-jbx-docs.json",
        br#"{"artifact":"dev.telegraphic:demo:1.0.0","summary":"Remote docs"}"#.to_vec(),
    )]));

    let out = jbx_command()
        .arg("docs")
        .arg("dev.telegraphic:demo:1.0.0")
        .arg("--json")
        .arg("--repo")
        .arg(format!("local={repo}"))
        .arg("--cache-dir")
        .arg(tmp.path().join("cache"))
        .output()
        .unwrap();

    assert_success(&out);
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["artifact"], "dev.telegraphic:demo:1.0.0");
    assert_eq!(json["summary"], "Remote docs");
}
