extern crate assert_cli;

const NAME: &str = "kupc2015_a";

#[test]
fn test_0() {
    let stdin = r#"
3
higashikyoto
kupconsitetokyotokyoto
goodluckandhavefun
    "#;
    let stdout = r#"
1
2
0
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is(stdout)
        .unwrap();
}
