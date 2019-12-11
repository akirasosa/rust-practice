extern crate assert_cli;

const NAME: &str = "arc005_3";

#[test]
fn test_0() {
    let stdin = r#"
4 5
s####
....#
#####
#...g
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("YES")
        .unwrap();
}

#[test]
fn test_1() {
    let stdin = r#"
4 4
...s
....
....
.g..
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("YES")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
10 10
s.........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
g##.#.#.#.
###.#.#.#.
###.#.#.#.
#.....#...
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("YES")
        .unwrap();
}

#[test]
fn test_3() {
    let stdin = r#"
6 6
.....s
###...
###...
######
...###
g.####
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("YES")
        .unwrap();
}

#[test]
fn test_5() {
    let stdin = r#"
1 10
s..####..g
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("NO")
        .unwrap();
}
