extern crate assert_cli;

const NAME: &str = "abc104_c";

#[test]
fn test_0() {
    let stdin = r#"
2 700
3 500
5 800
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
2 2000
3 500
5 800
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("7")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
2 400
3 500
5 800
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
5 25000
20 1000
40 1000
50 1000
30 1000
1 1000
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("66")
        .unwrap();
}
