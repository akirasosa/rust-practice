extern crate assert_cli;

const NAME: &str = "abc015_4";

#[test]
fn test_0() {
    let stdin = r#"
10
3 2
4 20
3 40
6 100
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("140")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
10
5 4
9 10
3 7
3 1
2 6
4 5
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("18")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
22
5 3
5 40
8 50
3 60
4 70
6 80
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("210")
        .unwrap();
}
