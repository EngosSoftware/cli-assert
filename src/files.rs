use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
  /// Temporary directory.
  dir: TempDir,
  /// Full path to file in temporary directory.
  path: PathBuf,
}

impl TempFile {
  /// Creates a new file with specified name in temporary directory.
  pub fn new(file_name: impl AsRef<Path>) -> Self {
    let dir = TempDir::default();
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

pub struct TempDir {
  path: PathBuf,
}

impl Default for TempDir {
  fn default() -> Self {
    Self::new()
  }
}

impl TempDir {
  /// Creates a new temporary directory.
  pub fn new() -> Self {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let pid = std::process::id();
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let name = format!("cli-assert-{pid}-{nanos}-{count}");
    let path = std::env::temp_dir().join(name);
    std::fs::create_dir(&path).unwrap();
    TempDir { path }
  }

  /// Returns a path to temporary directory.
  pub fn path(&self) -> &Path {
    &self.path
  }
}

impl Drop for TempDir {
  fn drop(&mut self) {
    _ = std::fs::remove_dir_all(&self.path);
  }
}
