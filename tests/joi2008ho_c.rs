extern crate assert_cli;

const NAME: &str = "joi2008ho_c";

#[test]
fn test_0() {
    let stdin = r#"
4 50
3
14
15
9
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("48")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
3 21
16
11
2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("20")
        .unwrap();
}
