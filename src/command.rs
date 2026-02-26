use crate::assertions::Assertions;
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
  /// Assertions to be checked after executing this command.
  assertions: Assertions,
}

impl Command {
  pub fn new(program: impl AsRef<OsStr>, caller_file: impl AsRef<str>, manifest_dir: impl AsRef<str>) -> Self {
    let manifest_path = Path::new(manifest_dir.as_ref());
    let caller_path = Path::new(caller_file.as_ref()).parent().expect("failed to retrieve parent directory for caller file");
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
      assertions: Default::default(),
    }
  }

  pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
    self.current_dir = dir.as_ref().into();
    self
  }

  pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
    self.program_args.push(arg.as_ref().into());
    self
  }

  pub fn stdin(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.stdin = Some(bytes.as_ref().to_vec());
    self
  }

  pub fn success(mut self) -> Self {
    self.assertions.success();
    self
  }

  pub fn failure(mut self) -> Self {
    self.assertions.failure();
    self
  }

  pub fn code(mut self, code: i32) -> Self {
    self.assertions.code(code);
    self
  }

  pub fn code_fn(mut self, predicate: impl Fn(i32) -> bool + 'static) -> Self {
    self.assertions.code_fn(predicate);
    self
  }

  pub fn stdout(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.assertions.stdout(bytes);
    self
  }

  pub fn stdout_fn(mut self, predicate: impl Fn(Vec<u8>) -> bool + 'static) -> Self {
    self.assertions.stdout_fn(predicate);
    self
  }

  pub fn stderr(mut self, bytes: impl AsRef<[u8]>) -> Self {
    self.assertions.stderr(bytes);
    self
  }

  pub fn stderr_fn(mut self, predicate: impl Fn(Vec<u8>) -> bool + 'static) -> Self {
    self.assertions.stderr_fn(predicate);
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
      .expect("failed to spawn requested command");
    if let Some(bytes) = &self.stdin {
      let mut stdin = child.stdin.take().expect("failed to obtain child process stdin");
      stdin.write_all(bytes).expect("failed to write child process stdin");
    }
    self.child = Some(child);
  }

  pub fn wait(&mut self) {
    let child = self.child.take().expect("command is not spawned");
    let output = child.wait_with_output().expect("failed to obtain child process output");
    self.stdout = output.stdout;
    self.stderr = output.stderr;
    self.status = output.status;
    self.assertions.assert(self);
  }

  pub fn execute(&mut self) {
    self.spawn();
    self.wait();
  }

  pub fn stop(&mut self) {
    if let Some(child) = &mut self.child {
      child.kill().expect("failed to force a child process to stop");
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
}
