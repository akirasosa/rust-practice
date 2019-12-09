extern crate assert_cli;

const NAME: &str = "abc051_b";

#[test]
fn test_0() {
    let stdin = r#"
2 2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("6")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
5 15
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}
