use std::fs;
use std::process::Command;

const PACK: &str = "\
5  389ed6887e49a315f706f6c2b931b1dcf0d797c91437124f32eb98555c669758  docs/note.txt
5  2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  hello.txt
";

#[test]
fn pack_manifest_matches_the_readme() {
    let output = Command::new(env!("CARGO_BIN_EXE_roster"))
        .arg("fixtures/pack")
        .output()
        .expect("run roster");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, PACK);
    let readme = fs::read_to_string("README.md").unwrap();
    assert!(readme.contains(PACK.trim()));
    let again = Command::new(env!("CARGO_BIN_EXE_roster"))
        .arg("fixtures/pack")
        .output()
        .unwrap();
    assert_eq!(String::from_utf8(again.stdout).unwrap(), PACK);
}
