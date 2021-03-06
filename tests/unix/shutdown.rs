use assert_cmd::prelude::*;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::helper::*;

#[test]
/// Spin up the daemon and send a SIGTERM shortly afterwards.
/// This should trigger the graceful shutdown and kill the process.
fn test_ctrlc() -> Result<()> {
    let (_settings, tempdir) = base_setup()?;

    let mut child = Command::cargo_bin("pueued")?
        .arg("--config")
        .arg(tempdir.path().join("pueue.yml").to_str().unwrap())
        .arg("-vvv")
        .stdout(Stdio::piped())
        .spawn()?;

    let pid = get_pid(tempdir.path())?;

    use nix::sys::signal::{kill, Signal};
    // Send SIGTERM signal to process via nix
    let nix_pid = nix::unistd::Pid::from_raw(pid);
    kill(nix_pid, Signal::SIGTERM).context("Failed to send SIGTERM to pid")?;

    // Sleep for 500ms and give the daemon time to shut down
    sleep_ms(500);

    let result = child.try_wait();
    assert!(matches!(result, Ok(Some(_))));
    let code = result.unwrap().unwrap();
    assert!(matches!(code.code(), Some(1)));

    Ok(())
}
