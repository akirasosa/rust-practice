extern crate assert_cli;

const NAME: &str = "joi2010yo_d";

#[test]
fn test_0() {
    let stdin = r#"
4
2
1
2
12
1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("7")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
6
3
72
2
12
7
2
1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("68")
        .unwrap();
}
