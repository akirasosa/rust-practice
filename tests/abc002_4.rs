extern crate assert_cli;

const NAME: &str = "abc002_4";

#[test]
fn test_0() {
    let stdin = r#"
5 3
1 2
2 3
1 3
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
5 3
1 2
2 3
3 4
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
7 9
1 2
1 3
2 3
4 5
4 6
4 7
5 6
5 7
6 7
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
12 0
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}
