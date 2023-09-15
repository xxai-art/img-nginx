use std::process::Command;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  Sh(String),
}

pub fn sh(command_str: &str) -> Result<String> {
  let mut parts: Vec<&str> = command_str.split_whitespace().collect();

  let command = parts.remove(0);

  let output = Command::new(command).args(&parts).output();

  match output {
    Ok(output) => {
      if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
      } else {
        Err(Error::Sh(String::from_utf8_lossy(&output.stderr).into_owned()).into())
      }
    }
    Err(e) => Err(Error::Sh(e.to_string()).into()),
  }
}
