extern crate assert_cli;

const NAME: &str = "tdpc_dice";

#[test]
fn test_0() {
    let stdin = r#"
2 6
    "#;
    assert_cli::Assert::cargo_binary(NAME)
        .stdin(stdin)
        .stdout()
        .satisfies(|x| {
            let x = x.trim().parse::<f64>().unwrap();
            (x - 15. / 36.).abs() <= 1e-6
        }, "failed")
        .unwrap();
}
