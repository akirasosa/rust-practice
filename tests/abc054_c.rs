extern crate assert_cli;

const NAME: &str = "abc054_c";

#[test]
fn test_0() {
    let stdin = r#"
3 3
1 2
1 3
2 3
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
7 7
1 3
2 7
3 4
4 5
4 6
5 6
6 7
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}
