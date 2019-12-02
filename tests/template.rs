extern crate assert_cli;

const NAME: &str = "xxxxx";

#[test]
fn test_0() {
    let stdin = r#"
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
//        .is("3")
        .unwrap();
}
