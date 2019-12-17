extern crate assert_cli;

const NAME: &str = "joi2013yo_d";

#[test]
fn test_0() {
    let stdin = r#"
3 4
31
27
35
20 25 30
23 29 90
21 35 60
28 33 40
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("80")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
5 2
26
28
32
29
34
30 35 0
25 30 100
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("300")
        .unwrap();
}
