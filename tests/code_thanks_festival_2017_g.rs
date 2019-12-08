extern crate assert_cli;

const NAME: &str = "code_thanks_festival_2017_g";

#[test]
fn test_0() {
    let stdin = r#"
4 1
1 2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
