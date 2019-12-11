extern crate assert_cli;

const NAME: &str = "joi2008yo_a";

#[test]
fn test_0() {
    let stdin = r#"
380
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("15")
        .unwrap();
}
