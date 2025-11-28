use crate::utils::PathExt;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, ExitStatus};

pub struct Command {
  /// Path to the program to be executed.
  program: OsString,
  /// Arguments passed to spawned program.
  program_args: Vec<OsString>,
  /// Path to the directory this command is executed in.
  current_dir: PathBuf,
  /// Child process.
  child: Option<Child>,
  /// The raw content to be written to stdin of the child process.
  stdin: Option<Vec<u8>>,
  /// The raw content of the stdout of the child process.
  stdout: Vec<u8>,
  /// The raw content of the stderr of the child process.
  stderr: Vec<u8>,
  /// Execution status of the child process.
  status: ExitStatus,
  /// Flag indicating if the command is expected to complete successfully.
  expected_success: Option<bool>,
  /// Flag indicating if the command is expected to fail.
  expected_failure: Option<bool>,
  /// Expected status code.
  expected_status: Option<i32>,
  /// Expected stdout.
  expected_stdout: Option<Vec<u8>>,
  /// Expected stderr.
  expected_stderr: Option<Vec<u8>>,
}

impl Command {
  pub fn new(program: impl AsRef<OsStr>, caller_file: impl AsRef<str>, manifest_dir: impl AsRef<str>) -> Self {
    let manifest_path = Path::new(manifest_dir.as_ref());
    let caller_path = Path::new(caller_file.as_ref()).parent().unwrap();
    // .expect("failed to retrieve parent directory for caller file");
    let current_dir = match manifest_path.rem(caller_path) {
      None => caller_path.into(),
      Some(path_buf) => {
        if path_buf.components().count() == 0 {
          PathBuf::from(".")
        } else {
          path_buf
        }
      }
    };
    Self {
      program: program.as_ref().into(),
      program_args: vec![],
      current_dir,
      child: None,
      stdin: None,
      stdout: vec![],
      stderr: vec![],
      status: ExitStatus::default(),
      expected_success: None,
      expected_failure: None,
      expected_status: None,
      expected_stdout: None,
      expected_stderr: None,
    }
  }

  pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
    self.program_args.push(arg.as_ref().into());
    self
  }

  pub fn success(mut self) -> Self {
    self.expected_success = Some(true);
    self.expected_failure = None;
    self
  }

  pub fn failure(mut self) -> Self {
    self.expected_failure = Some(true);
    self.expected_success = None;
    self
  }

  pub fn code(mut self, code: i32) -> Self {
    self.expected_status = Some(code);
    self
  }

  pub fn stdin(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.stdin = Some(bytes.as_ref().to_vec());
    self
  }

  pub fn stdout(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.expected_stdout = Some(bytes.as_ref().to_vec());
    self
  }

  pub fn stderr(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.expected_stderr = Some(bytes.as_ref().to_vec());
    self
  }

  pub fn spawn(&mut self) {
    if self.child.is_some() {
      panic!("command is already spawned");
    }
    let mut command = std::process::Command::new(self.program.clone());
    let mut child = command
      .args(self.program_args.clone())
      .current_dir(self.current_dir.clone())
      .stdin(std::process::Stdio::piped())
      .stdout(std::process::Stdio::piped())
      .stderr(std::process::Stdio::piped())
      .spawn()
      .unwrap();
    //.expect("failed to spawn requested command");
    if let Some(bytes) = &self.stdin {
      let mut stdin = child.stdin.take().unwrap(); //.expect("failed to obtain the stdin handle");
      stdin.write_all(bytes).expect("failed to write stdin");
    }
    self.child = Some(child);
  }

  pub fn wait(&mut self) {
    if let Some(child) = self.child.take() {
      let output = child.wait_with_output().unwrap();
      // .expect("failed during waiting for a child process");
      self.stdout = output.stdout;
      self.stderr = output.stderr;
      self.status = output.status;
      self.assert();
    } else {
      panic!("command is not spawned");
    }
  }

  pub fn execute(&mut self) {
    self.spawn();
    self.wait();
  }

  pub fn stop(&mut self) {
    if let Some(child) = &mut self.child {
      child.kill().unwrap(); //.expect("failed to force a child process to stop");
    } else {
      panic!("command is not spawned");
    }
  }

  pub fn get_program(&self) -> &OsStr {
    &self.program
  }

  pub fn get_current_dir(&self) -> &Path {
    &self.current_dir
  }

  pub fn get_stdout(&'_ self) -> Cow<'_, str> {
    String::from_utf8_lossy(&self.stdout)
  }

  pub fn get_stderr(&'_ self) -> Cow<'_, str> {
    String::from_utf8_lossy(&self.stderr)
  }

  pub fn get_stdout_raw(&self) -> &[u8] {
    &self.stdout
  }

  pub fn get_stderr_raw(&self) -> &[u8] {
    &self.stderr
  }

  pub fn get_status(&self) -> ExitStatus {
    self.status
  }

  /// Checks all assertions.
  fn assert(&self) {
    if let Some(true) = self.expected_success {
      assert!(self.status.success(), "expected success");
    }
    if let Some(true) = self.expected_failure {
      assert!(!self.status.success(), "expected failure");
    }
    if let Some(expected) = self.expected_status {
      let actual = self.status.code().expect("failed to retrieve status code");
      assert_eq!(
        expected, actual,
        "\nexpected status code: {}\n  actual status code: {}",
        expected, actual
      );
    }
    if let Some(expected) = &self.expected_stdout {
      let actual = self.get_stdout_raw();
      assert_eq!(
        expected, actual,
        "\nexpected stdout: {:?}\n  actual stdout: {:?}",
        expected, actual
      )
    }
    if let Some(expected) = &self.expected_stderr {
      let actual = self.get_stderr_raw();
      assert_eq!(
        expected, actual,
        "\nexpected stderr: {:?}\n  actual stderr: {:?}",
        expected, actual
      )
    }
  }
}
