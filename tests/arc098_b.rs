extern crate assert_cli;

const NAME: &str = "arc098_b";

#[test]
fn test_0() {
    let stdin = r#"
4
2 5 4 6
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("5")
        .unwrap();
}
