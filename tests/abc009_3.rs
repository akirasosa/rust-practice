extern crate assert_cli;

const NAME: &str = "abc009_3";

#[test]
fn test_0() {
    let stdin = r#"
3 2
abc
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("abc")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
7 2
atcoder
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("actoder")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
7 7
atcoder
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("acdeort")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
10 3
helloworld
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("dehloworll")
        .unwrap();
}
