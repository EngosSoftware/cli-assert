use std::path::{Component, Path, PathBuf};

pub trait PathExt {
  fn rem(&self, p: impl AsRef<Path>) -> Option<PathBuf>;
}

impl PathExt for Path {
  fn rem(&self, p: impl AsRef<Path>) -> Option<PathBuf> {
    let left: Vec<Component> = self.components().collect();
    let right: Vec<Component> = p.as_ref().components().collect();
    let max_length = left.len().min(right.len());
    for index in (1..=max_length).rev() {
      if left[left.len() - index..] == right[..index] {
        let mut path_buf = PathBuf::new();
        for component in &right[index..] {
          path_buf.push(component);
        }
        return Some(path_buf);
      }
    }
    None
  }
}

/// Pauses thread execution for specified number of milliseconds.
pub fn sleep(millis: u64) {
  if millis > 0 {
    std::thread::sleep(std::time::Duration::from_millis(millis));
  }
}

/// Converts UTF-8 bytes to string.
pub fn bytes_to_str(bytes: impl AsRef<[u8]>) -> String {
  let mut s = String::from_utf8(bytes.as_ref().to_vec()).unwrap();
  if s.is_empty() {
    s = "(empty)".to_string();
  }
  s
}
