extern crate assert_cli;

const NAME: &str = "arc017_3";

#[test]
fn test_0() {
    let stdin = r#"
5 5
1
1
2
3
4
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
8 21
10
4
2
30
22
20
8
14
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn test_2() {
    let stdin = r#"
20 100000000
35576749
38866484
6624951
39706038
11133516
25490903
14701702
17888322
14423251
32111678
24509097
43375049
35298298
21158709
30489274
37977301
19510726
28841291
10293962
12022699
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("45")
        .unwrap();
}
#[test]

fn test_3() {
    let stdin = r#"
16 8
1
1
1
1
1
1
1
1
1
1
1
1
1
1
1
1
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("12870")
        .unwrap();
}
