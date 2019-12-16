extern crate assert_cli;

const NAME: &str = "tdpc_contest";

#[test]
fn test_0() {
    let stdin = r#"
4
0 2 3 5
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("7")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
10
1 1 1 1 1 1 1 1 1 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("11")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
100
100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100 100
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("101")
        .unwrap();
}
