extern crate assert_cli;

const NAME: &str = "maximum_cup_2018_d";

#[test]
fn test_0() {
    let stdin = r#"
5 11 7 2
1 4 5 8 9
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("Yes")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
5 5 3 2
1 4 5 9 12
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("No")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
5 10 3 100
1 4 7 10 14
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("No")
        .unwrap();
}
