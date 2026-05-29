use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::{Command, Output};
use std::sync::mpsc;
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

fn serve_search_response(
    body: &'static str,
) -> (String, mpsc::Receiver<String>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let read = stream.read(&mut request).unwrap_or(0);
        let request_text = String::from_utf8_lossy(&request[..read]).to_string();
        tx.send(request_text).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body
        );
        stream.write_all(response.as_bytes()).unwrap();
    });
    (base, rx, handle)
}

#[test]
fn search_prints_human_readable_maven_central_results() {
    let (base, requests, handle) = serve_search_response(
        r#"{
  "response": {
    "numFound": 1,
    "docs": [
      {"id":"com.google.inject:guice","g":"com.google.inject","a":"guice","latestVersion":"7.0.0","p":"jar","versionCount": 24}
    ]
  }
}"#,
    );

    let output = jbx_command()
        .arg("search")
        .arg("guice")
        .arg("--limit")
        .arg("5")
        .env("JBX_MAVEN_SEARCH_URL", base)
        .output()
        .expect("failed to run jbx search");

    assert_success(&output);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "com.google.inject:guice\t7.0.0\tjar\t24 versions"
    );
    let request = requests.recv().unwrap();
    handle.join().unwrap();
    assert!(request.starts_with("GET /solrsearch/select?"), "{request}");
    assert!(request.contains("q=guice"), "{request}");
    assert!(request.contains("rows=5"), "{request}");
    assert!(request.contains("wt=json"), "{request}");
}

#[test]
fn search_json_outputs_agent_friendly_payload_and_coordinate_query() {
    let (base, requests, handle) = serve_search_response(
        r#"{
  "response": {
    "numFound": 1,
    "docs": [
      {"id":"com.google.inject:guice:7.0.0","g":"com.google.inject","a":"guice","v":"7.0.0","p":"jar","timestamp": 1684863927000}
    ]
  }
}"#,
    );

    let output = jbx_command()
        .arg("search")
        .arg("com.google.inject:guice:7.0.0")
        .arg("--json")
        .env("JBX_MAVEN_SEARCH_URL", base)
        .output()
        .expect("failed to run jbx search --json");

    assert_success(&output);
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        payload["query"],
        "g:com.google.inject AND a:guice AND v:7.0.0"
    );
    assert_eq!(payload["numFound"], 1);
    assert_eq!(
        payload["artifacts"][0]["coordinate"],
        "com.google.inject:guice:7.0.0"
    );
    assert_eq!(payload["artifacts"][0]["groupId"], "com.google.inject");
    assert_eq!(payload["artifacts"][0]["artifactId"], "guice");
    assert_eq!(payload["artifacts"][0]["version"], "7.0.0");

    let request = requests.recv().unwrap();
    handle.join().unwrap();
    assert!(request.contains("core=gav"), "{request}");
    assert!(
        request.contains("q=g%3Acom.google.inject%20AND%20a%3Aguice%20AND%20v%3A7.0.0")
            || request.contains("q=g%3Acom.google.inject+AND+a%3Aguice+AND+v%3A7.0.0"),
        "{request}"
    );
}
