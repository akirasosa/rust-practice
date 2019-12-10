extern crate assert_cli;

const NAME: &str = "arc037_b";

#[test]
fn test_0() {
    let stdin = r#"
8 7
1 2
2 3
2 4
5 6
6 7
6 8
7 8
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
5 1
1 2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
11 11
1 2
1 3
2 4
3 5
4 6
5 7
6 8
7 9
8 10
9 11
10 11
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("0")
        .unwrap();
}
