extern crate assert_cli;

const NAME: &str = "agc013_c";

#[test]
fn test_0() {
    let stdin = r#"
3 8 3
0 1
3 2
6 1
    "#;
    let stdout = r#"
1
3
0
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is(stdout)
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
4 20 9
7 2
9 1
12 1
18 1
   "#;
    let stdout = r#"
7
18
18
1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is(stdout)
        .unwrap();
}
