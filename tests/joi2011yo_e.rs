extern crate assert_cli;

const NAME: &str = "joi2011yo_e";

#[test]
fn test_0() {
    let stdin = r#"
3 3 1
S..
...
..1
   "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
4 5 2
.X..1
....X
.XX.S
.2.X.
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("12")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
10 10 9
.X...X.S.X
6..5X..X1X
...XXXX..X
X..9X...X.
8.X2X..X3X
...XX.X4..
XX....7X..
X..X..XX..
X...X.XX..
..X.......
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("91")
        .unwrap();
}
