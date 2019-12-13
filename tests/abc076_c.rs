extern crate assert_cli;

const NAME: &str = "abc076_c";

#[test]
fn test_0() {
    let stdin = r#"
?tc????
coder
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("atcoder")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
??p??d??
abc
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("UNRESTORABLE")
        .unwrap();
}
