extern crate assert_cli;

#[test]
fn test_0() {
    let stdin = r#"
    3 2
    100 15
    300 20
    200 30
    "#;
    assert_cli::Assert::cargo_binary("abc034_d")
        .stdin(stdin)
        .stdout().is("25")
        .unwrap();
}
