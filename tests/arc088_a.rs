extern crate assert_cli;

const NAME: &str = "arc088_a";

#[test]
fn test_0() {
    let stdin = r#"
3 20
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
25 100
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
