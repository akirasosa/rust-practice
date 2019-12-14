extern crate assert_cli;

const NAME: &str = "arc006_3";

#[test]
fn test_0() {
    let stdin = r#"
5
4
3
1
2
1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("2")
        .unwrap();
}
