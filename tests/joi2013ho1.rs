extern crate assert_cli;

const NAME: &str = "joi2013ho1";

#[test]
fn test_0() {
    let stdin = r#"
10
1 1 0 0 1 0 1 1 1 0
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
1 0 0 0 0 1 0 1 0 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("8")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
5
1 1 0 1 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("5")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
3
0 1 0
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
