extern crate assert_cli;

const NAME: &str = "arc004_1";

#[test]
fn test_0() {
    let stdin = r#"
3
1 1
2 4
4 3
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("")
        .unwrap();
}
