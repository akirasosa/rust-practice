extern crate assert_cli;

const NAME: &str = "arc029_1";

#[test]
fn test_0() {
    let stdin = r#"
4
4
6
7
10
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("14")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
3
1
2
4
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
1
29
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("29")
        .unwrap();
}
