extern crate assert_cli;

const NAME: &str = "abc088_d";

#[test]
fn test_0() {
    let stdin = r#"
3 3
..#
#..
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
10 37
.....................................
...#...####...####..###...###...###..
..#.#..#...#.##....#...#.#...#.#...#.
..#.#..#...#.#.....#...#.#...#.#...#.
.#...#.#..##.#.....#...#.#.###.#.###.
.#####.####..#.....#...#..##....##...
.#...#.#...#.#.....#...#.#...#.#...#.
.#...#.#...#.##....#...#.#...#.#...#.
.#...#.####...####..###...###...###..
.....................................
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("209")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
3 3
..#
###
...
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("-1")
        .unwrap();
}
