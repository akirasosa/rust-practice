extern crate assert_cli;

const NAME: &str = "abc017_4";

#[test]
fn test_0() {
    let stdin = r#"
5 2
1
2
1
2
2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("5")
        .unwrap();
}
