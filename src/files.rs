use std::path::{Path, PathBuf};

pub struct TmpFile {
  /// Temporary directory.
  dir: tempfile::TempDir,
  /// Full path to file in temporary directory.
  path: PathBuf,
}

impl TmpFile {
  /// Creates a new file with specified name.
  pub fn new(file_name: impl AsRef<Path>) -> Self {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join(file_name);
    std::fs::File::create_new(&path).unwrap();
    Self { dir, path }
  }

  /// Writes data to file.
  pub fn write(&self, data: impl AsRef<[u8]>) {
    std::fs::write(&self.path, data.as_ref()).unwrap();
  }

  /// Asserts the expected file content.
  pub fn assert(&self, expected: impl AsRef<[u8]>) {
    let actual = std::fs::read(&self.path).unwrap();
    let expected = expected.as_ref();
    if actual != expected {
      println!("expected content: {:?}", expected);
      println!("  actual content: {:?}", actual);
      panic!("unexpected content")
    }
  }

  /// Returns the directory name.
  pub fn dir(&self) -> &Path {
    self.dir.path()
  }

  /// Returns the file path.
  pub fn path(&self) -> &Path {
    &self.path
  }

  /// Sets read-only permission on file.
  pub fn set_readonly(&self, readonly: bool) {
    let mut permissions = std::fs::metadata(self.path()).unwrap().permissions();
    permissions.set_readonly(readonly);
    std::fs::set_permissions(self.path(), permissions).unwrap();
  }
}
