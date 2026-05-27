use std::fs;
use std::process::{Command, Output};

fn juv_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_juv"))
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn run_resolves_alias_from_local_jbang_catalog() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(
        tmp.path().join("Hello.java"),
        r#"class Hello {
  public static void main(String[] args) {
    System.out.println("hello " + args[0] + " " + args[1]);
  }
}
"#,
    )
    .unwrap();
    fs::write(
        tmp.path().join("jbang-catalog.json"),
        r#"{
  "aliases": {
    "hello": {
      "script-ref": "Hello.java",
      "arguments": ["from-catalog"]
    }
  }
}
"#,
    )
    .unwrap();

    let output = juv_command()
        .current_dir(tmp.path())
        .arg("run")
        .arg("hello")
        .arg("from-cli")
        .output()
        .unwrap();

    assert_success(&output);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "hello from-catalog from-cli\n"
    );
}

#[test]
fn shorthand_resolves_alias_from_parent_catalog_with_base_ref() {
    let tmp = tempfile::tempdir().unwrap();
    let nested = tmp.path().join("nested");
    fs::create_dir_all(tmp.path().join("scripts")).unwrap();
    fs::create_dir_all(&nested).unwrap();
    fs::write(
        tmp.path().join("scripts/Tool.java"),
        r#"class Tool {
  public static void main(String[] args) {
    System.out.println("tool " + args[0]);
  }
}
"#,
    )
    .unwrap();
    fs::write(
        tmp.path().join("jbang-catalog.json"),
        r#"{
  "base-ref": "scripts",
  "aliases": {
    "tool": { "script-ref": "Tool.java" }
  }
}
"#,
    )
    .unwrap();

    let output = juv_command()
        .current_dir(&nested)
        .arg("tool")
        .arg("works")
        .output()
        .unwrap();

    assert_success(&output);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "tool works\n");
}

#[test]
fn alias_list_prints_local_catalog_aliases() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(
        tmp.path().join("jbang-catalog.json"),
        r#"{
  "aliases": {
    "alpha": { "script-ref": "Alpha.java", "description": "Alpha script" },
    "beta": { "script-ref": "tools/Beta.java" }
  }
}
"#,
    )
    .unwrap();

    let output = juv_command()
        .current_dir(tmp.path())
        .arg("alias")
        .arg("list")
        .output()
        .unwrap();

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("alpha\tAlpha.java\tAlpha script"),
        "{stdout}"
    );
    assert!(stdout.contains("beta\ttools/Beta.java"), "{stdout}");
}
