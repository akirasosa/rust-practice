extern crate assert_cli;

#[test]
fn test_0() {
    assert_cli::Assert::cargo_binary("abc086_a")
        .stdin("5 8")
        .stdout().is("Even")
        .unwrap();
}
