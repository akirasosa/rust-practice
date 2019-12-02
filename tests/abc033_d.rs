extern crate assert_cli;

const NAME: &str = "abc033_d";

#[test]
fn test_0() {
    let stdin = r#"
5
1 3
2 2
3 2
4 1
4 3
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1 2 7")
        .unwrap();
}
