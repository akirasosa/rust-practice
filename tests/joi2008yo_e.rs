extern crate assert_cli;

const NAME: &str = "joi2008yo_e";

#[test]
fn test_0() {
    let stdin = r#"
2 5
0 1 0 1 0
1 0 0 0 1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("9")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
3 6
1 0 0 0 1 0
1 1 1 0 1 0
1 0 1 1 0 1
   "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("15")
        .unwrap();
}
