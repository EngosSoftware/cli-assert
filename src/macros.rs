/// Returns the absolute path to the directory containing Cargo manifest file.
#[macro_export]
macro_rules! cargo_manifest_dir {
  () => {
    env!("CARGO_MANIFEST_DIR")
  };
}

/// Returns the name of the Cargo package.
#[macro_export]
macro_rules! cargo_package {
  () => {
    env!("CARGO_PKG_NAME")
  };
}

/// Returns the absolute path to the target's executable file.
#[macro_export]
macro_rules! cargo_binary {
  () => {
    $crate::cargo_binary!($crate::cargo_package!())
  };
  ($name:expr) => {
    env!(concat!("CARGO_BIN_EXE_", $name))
  };
}

/// Returns a new command-line command for executing tested application.
#[macro_export]
macro_rules! command {
  () => {
    $crate::Command::new($crate::cargo_binary!(), file!(), $crate::cargo_manifest_dir!())
  };
  ($name:expr) => {
    $crate::Command::new($crate::cargo_binary!($name), file!(), $crate::cargo_manifest_dir!())
  };
}

/// Returns a new command-line command for any application.
#[macro_export]
macro_rules! cmd {
  ($name:expr) => {
    $crate::Command::new($name, file!(), $crate::cargo_manifest_dir!())
  };
}
