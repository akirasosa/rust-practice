extern crate assert_cli;

const NAME: &str = "arc075_c";

#[test]
fn test_0() {
    let stdin = r#"
3 6
7
5
7
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("5")
        .unwrap();
}
