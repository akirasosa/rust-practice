extern crate assert_cli;

const NAME: &str = "arc061_a";

#[test]
fn test_0() {
    let stdin = r#"
125
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("176")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
9999999999
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("12656242944")
        .unwrap();
}
