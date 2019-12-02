extern crate assert_cli;

#[test]
fn test_0() {
    let stdin = r#"
3 2
0 1 5 3
1 2 5 2
    "#;
    assert_cli::Assert::cargo_binary("arc026_4")
        .stdin(stdin)
        .stdout()
        .satisfies(|x| {
            let x = x.trim().parse::<f64>().unwrap();
            (x - 2.).abs() <= 1e-2
        }, "failed")
        .unwrap();
}
