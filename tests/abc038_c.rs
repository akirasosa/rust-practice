extern crate assert_cli;

const NAME: &str = "abc038_c";

#[test]
fn test_0() {
    let stdin = r#"
5
1 2 3 2 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("8")
        .unwrap();
}
