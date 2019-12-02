extern crate assert_cli;

const NAME: &str = "arc022_2";

#[test]
fn test_0() {
    let stdin = r#"
7
1 2 1 3 1 4 4
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
