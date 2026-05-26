use std::fs;
use std::process::Command;

#[test]
fn runs_single_java_file_with_jbang_style_main_directive() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("Hello.java");
    fs::write(
        &src,
        r#"
//MAIN Hello
class Hello {
  public static void main(String[] args) {
    System.out.println("hello " + args[0]);
  }
}
"#,
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_doj");
    let out = Command::new(bin)
        .arg("run")
        .arg(&src)
        .arg("world")
        .output()
        .unwrap();

    assert!(
        out.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "hello world");
}
