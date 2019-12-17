extern crate assert_cli;

const NAME: &str = "tdpc_number";

#[test]
fn test_0() {
    let stdin = r#"
3
100
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("33")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
7
123456789012345678901234567890
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("468357804")
        .unwrap();
}
