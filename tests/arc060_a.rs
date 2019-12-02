extern crate assert_cli;

const NAME: &str = "arc060_a";

#[test]
fn test_0() {
    let stdin = r#"
4 8
7 9 8 9
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("5")
        .unwrap();
}
