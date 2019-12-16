extern crate assert_cli;

const NAME: &str = "arc092_a";

#[test]
fn test_0() {
    let stdin = r#"
3
2 0
3 1
1 3
4 2
0 4
5 5
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
3
0 0
1 1
5 2
2 3
3 4
4 5
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
2
2 2
3 3
0 0
1 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
5
0 0
7 3
2 2
4 8
1 6
8 5
6 9
5 4
9 1
3 7
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("5")
        .unwrap();
}
