extern crate assert_cli;

const NAME: &str = "abc081_b";

#[test]
fn test_0() {
    let stdin = r#"
3
8 12 40
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("2")
        .unwrap();
}
