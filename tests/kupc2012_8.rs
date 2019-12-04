extern crate assert_cli;

const NAME: &str = "kupc2012_8";

#[test]
fn test_0() {
    let stdin = r#"
4 4
0 0 0 1
0 1 1 0
0 1 1 0
1 0 0 0
    "#;
    let stdout = r#"
1 0 0 0
0 0 0 0
0 0 0 0
0 0 0 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is(stdout)
        .unwrap();
}
