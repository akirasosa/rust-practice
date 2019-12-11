extern crate assert_cli;

const NAME: &str = "abc103_d";

#[test]
fn test_0() {
    let stdin = r#"
5 2
1 4
2 5
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
9 5
1 8
2 7
3 5
4 6
7 9
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
5 10
1 2
1 3
1 4
1 5
2 3
2 4
2 5
3 4
3 5
4 5
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}
