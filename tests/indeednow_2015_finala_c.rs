extern crate assert_cli;

const NAME: &str = "indeednow_2015_finala_c";

#[test]
fn test_0() {
    let stdin = r#"
3 6
1 2 3 3
3 3 3 6
4 4 4 8
3 4 3
4 4 4
100 100 1
2 3 4
0 0 0
100 100 100
    "#;
    let stdout = r#"
6
8
0
3
0
8
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is(stdout)
        .unwrap();
}
