extern crate assert_cli;

const NAME: &str = "codefestival_2015_qual_a_d";

#[test]
fn test_0() {
    let stdin = r#"
17 5
1
5
10
15
16
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .is("3")
        .unwrap();
}
