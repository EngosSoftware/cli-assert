use crate::utils::PathExt;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
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
  pub fn new(program: impl AsRef<OsStr>, caller: &str, dir: &str) -> Self {
    let dir = Path::new(dir);
    let caller = Path::new(caller).parent().expect("failed to retrieve parent directory");
    let current_dir = match dir.rem(caller) {
      None => caller.into(),
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

  pub fn stdout(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.expected_stdout = Some(bytes.as_ref().to_vec());
    self
  }

  pub fn stderr(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.expected_stderr = Some(bytes.as_ref().to_vec());
    self
  }

  pub fn spawn(&mut self) {
    let mut command = std::process::Command::new(self.program.clone());
    self.child = Some(
      command
        .args(self.program_args.clone())
        .current_dir(self.current_dir.clone())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn requested command"),
    );
  }

  pub fn wait(&mut self) {
    if let Some(child) = self.child.take() {
      let output = child
        .wait_with_output()
        .expect("failed during waiting for a child process");
      self.stdout = output.stdout;
      self.stderr = output.stderr;
      self.status = output.status;
      self.assert();
    }
  }

  pub fn execute(&mut self) {
    self.spawn();
    self.wait();
  }

  pub fn stop(&mut self) {
    if let Some(child) = &mut self.child {
      child.kill().expect("failed to force a child process to stop");
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
    if let Some(expected) = self.expected_stdout.as_ref() {
      let actual = self.get_stdout_raw();
      assert_eq!(
        expected, actual,
        "\nexpected stdout: {:?}\n  actual stdout: {:?}",
        expected, actual
      )
    }
    if let Some(expected) = self.expected_stderr.as_ref() {
      let actual = self.get_stderr_raw();
      assert_eq!(
        expected, actual,
        "\nexpected stderr: {:?}\n  actual stderr: {:?}",
        expected, actual
      )
    }
  }
}
