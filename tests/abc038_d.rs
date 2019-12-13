extern crate assert_cli;

const NAME: &str = "abc038_d";

#[test]
fn test_0() {
    let stdin = r#"
3
3 3
1 1
2 2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
2
4 5
4 3
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
4
2 5
3 3
4 5
6 6
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
5
8 8
5 10
5 2
5 5
2 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
