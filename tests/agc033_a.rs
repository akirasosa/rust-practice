extern crate assert_cli;

const NAME: &str = "agc033_a";

#[test]
fn test_0() {
    let stdin = r#"
3 3
...
.#.
...
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
6 6
..#..#
......
#..#..
......
.#....
....#.
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
