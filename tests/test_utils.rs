use cli_assert::PathExt;
use std::path::Path;

#[test]
fn _0001() {
  let p = Path::new("/a/b/c/d");
  let s = Path::new("e/f");
  assert_eq!(None, p.rem(s));
}

#[test]
fn _0002() {
  let p = Path::new("/a/b/c/d");
  let s = Path::new("d/e/f");
  assert_eq!("e/f", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0003() {
  let p = Path::new("a/b/c/d/e");
  let s = Path::new("d/e/f");
  assert_eq!("f", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0004() {
  let p = Path::new("a/b/c");
  let s = Path::new("e/f");
  assert_eq!(None, p.rem(s));
}

#[test]
fn _0005() {
  let p = Path::new("a/b/c");
  let s = Path::new("a/b/c");
  assert_eq!("", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0006() {
  let p = Path::new("a/b/c");
  let s = Path::new("b/c/d");
  assert_eq!("d", p.rem(s).unwrap().to_string_lossy());
}
