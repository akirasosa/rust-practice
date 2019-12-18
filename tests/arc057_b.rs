extern crate assert_cli;

const NAME: &str = "arc057_b";

#[test]
fn test_0() {
    let stdin = r#"
5 7
2
3
7
4
9
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
3 5
1
2
2
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
2 4
2
10
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
10 12
2
8
3
5
10
5
2
9
19
22
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("7")
        .unwrap();
}
