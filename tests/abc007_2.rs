extern crate assert_cli;

const NAME: &str = "abc007_2";

#[test]
fn test_0() {
    let stdin = r#"
xyz
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("a")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
a
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("-1")
        .unwrap();
}
