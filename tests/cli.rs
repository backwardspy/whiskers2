use assert_cmd::Command;

#[test]
fn test_single() {
    let mut cmd = Command::cargo_bin("whiskers2").expect("binary exists");
    let assert = cmd
        .args(["tests/fixtures/single/single.j2", "latte"])
        .assert();
    assert
        .success()
        .stdout(include_str!("fixtures/single/single.md"));
}

#[test]
fn test_multi() {
    let mut cmd = Command::cargo_bin("whiskers2").expect("binary exists");
    let assert = cmd.args(["tests/fixtures/multi/multi.j2"]).assert();
    assert
        .success()
        .stdout(include_str!("fixtures/multi/multi.md"));
}

#[test]
fn test_multifile_render() {
    let mut cmd = Command::cargo_bin("whiskers2").expect("binary exists");
    let assert = cmd
        .args(["--dry-run", "tests/fixtures/multifile.j2"])
        .assert();
    assert.success();
}
