extern crate assert_cli;

const NAME: &str = "arc064_a";

#[test]
fn test_0() {
    let stdin = r#"
3 3
2 2 2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}
