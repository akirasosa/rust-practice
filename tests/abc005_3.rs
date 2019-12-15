extern crate assert_cli;

const NAME: &str = "abc005_3";

#[test]
fn test_0() {
    let stdin = r#"
1
3
1 2 3
3
2 3 4
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("yes")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
1
3
1 2 3
3
2 3 5
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("no")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
1
3
1 2 3
10
1 2 3 4 5 6 7 8 9 10
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("no")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
1
3
1 2 3
3
1 2 2
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("no")
        .unwrap();
}

#[test]
fn test_4() {
    let stdin = r#"
2
5
1 3 6 10 15
3
4 8 16
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("yes")
        .unwrap();
}
