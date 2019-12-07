extern crate assert_cli;

const NAME: &str = "agc026_c";

#[test]
fn test_0() {
    let stdin = r#"
4
cabaacba
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
11
mippiisssisssiipsspiim
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("504")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
4
abcdefgh
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
18
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("9075135300")
        .unwrap();
}
