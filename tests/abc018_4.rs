extern crate assert_cli;

const NAME: &str = "abc018_4";

#[test]
fn test_0() {
    let stdin = r#"
3 4 2 3 7
1 1 9
1 2 7
1 3 15
1 4 6
2 2 3
2 4 6
3 3 6
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("37")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
4 5 3 2 9
2 3 5
3 1 4
2 2 2
4 1 9
3 5 3
3 3 8
1 4 5
1 5 7
2 4 8
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("26")
        .unwrap();
}
