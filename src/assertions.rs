use crate::utils::bytes_to_str;

#[derive(Default)]
pub struct Assertions {
  /// Flag indicating if the command is expected to complete successfully.
  success: Option<bool>,
  /// Flag indicating if the command is expected to fail.
  failure: Option<bool>,
  /// Expected status code.
  status: Option<i32>,
  /// Expected content of `stdout` after executing the command.
  stdout: Option<Vec<u8>>,
  /// Expected content of `stderr` after executing the command.
  stderr: Option<Vec<u8>>,
}

impl Assertions {
  pub fn success(&mut self) {
    self.success = Some(true);
    self.failure = None;
  }

  pub fn failure(&mut self) {
    self.failure = Some(true);
    self.success = None;
  }

  pub fn code(&mut self, code: i32) {
    self.status = Some(code);
  }

  pub fn stdout(&mut self, bytes: impl AsRef<[u8]>) {
    self.stdout = Some(bytes.as_ref().to_vec());
  }

  pub fn stderr(&mut self, bytes: impl AsRef<[u8]>) {
    self.stderr = Some(bytes.as_ref().to_vec());
  }

  /// Checks all assertions.
  pub fn assert(&self, command: &crate::Command) {
    if let Some(true) = self.success {
      if !command.get_status().success() {
        panic!("expected success");
      }
    }
    if let Some(true) = self.failure {
      if command.get_status().success() {
        panic!("expected failure");
      }
    }
    if let Some(expected) = self.status {
      let actual = command.get_status().code().expect("failed to retrieve status code");
      if actual != expected {
        println!("\nexpected status code: {}\n  actual status code: {}", expected, actual);
        panic!("unexpected status");
      }
    }
    if let Some(expected) = &self.stdout {
      let actual = command.get_stdout_raw();
      if actual != expected {
        println!("\nexpected stdout: {:?}\n  actual stdout: {:?}", expected, actual);
        println!("\n\nexpected stdout: {}\n  actual stdout: {}", bytes_to_str(expected), bytes_to_str(actual));
        panic!("unexpected stdout");
      }
    }
    if let Some(expected) = &self.stderr {
      let actual = command.get_stderr_raw();
      if actual != expected {
        println!("\nexpected stderr: {:?}\n  actual stderr: {:?}", expected, actual);
        println!("\n\nexpected stderr: {}\n  actual stderr: {}", bytes_to_str(expected), bytes_to_str(actual));
        panic!("unexpected stderr");
      }
    }
  }
}
