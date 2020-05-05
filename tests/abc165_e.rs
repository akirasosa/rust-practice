extern crate assert_cli;

const NAME: &str = "abc165_e";

#[test]
fn test_0() {
    let stdin = r#"
7 3
    "#;
    let stdout = r#"
1 4
2 3
7 5
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is(stdout)
        .unwrap();
}
