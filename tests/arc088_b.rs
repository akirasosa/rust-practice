extern crate assert_cli;

const NAME: &str = "arc088_b";

#[test]
fn test_0() {
    let stdin = r#"
010
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
100000000
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
00001111
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
100111100110
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("7")
        .unwrap();
}
