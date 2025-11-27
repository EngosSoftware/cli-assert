#[test]
fn cargo_package_name_macro_should_work() {
  assert_eq!("cli-assert", cli_assert::cargo_package!())
}
