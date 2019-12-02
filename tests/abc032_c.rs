extern crate assert_cli;

const NAME: &str = "abc032_c";

#[test]
fn test_0() {
    let stdin = r#"
7 6
4
3
1
1
2
10
2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}
