//! Run the `fileorz` binary under an isolated environment.

use crate::isolate::IsolatedHome;
use crate::paths::fileorz_bin;
use std::process::{Command, Output};

/// Captured stdout/stderr + status.
#[derive(Debug)]
pub struct CommandOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    fn from(output: Output) -> Self {
        Self {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }

    /// Assert exit code is zero.
    pub fn assert_ok(&self, ctx: &str) {
        assert_eq!(
            self.status, 0,
            "{ctx}: exit={} stdout={} stderr={}",
            self.status, self.stdout, self.stderr
        );
    }
}

/// Run `fileorz` with args under `home` isolation (no timeout).
pub fn run_fileorz(home: &IsolatedHome, args: &[&str]) -> std::io::Result<CommandOutput> {
    run_fileorz_capture(home, args, None)
}

/// Run `fileorz` with optional kill-after timeout (seconds).
pub fn run_fileorz_capture(
    home: &IsolatedHome,
    args: &[&str],
    timeout_secs: Option<u64>,
) -> std::io::Result<CommandOutput> {
    let bin = fileorz_bin();
    assert!(
        bin.is_file(),
        "missing fileorz binary at {} — run cargo build -p fileorz first",
        bin.display()
    );
    let mut cmd = if let Some(secs) = timeout_secs {
        let mut c = Command::new("timeout");
        c.arg(format!("{secs}s")).arg(&bin);
        c
    } else {
        Command::new(&bin)
    };
    home.apply(&mut cmd);
    cmd.args(args);
    Ok(CommandOutput::from(cmd.output()?))
}

/// Run `fileorz` with optional extra environment variables.
pub fn run_fileorz_env(
    home: &IsolatedHome,
    args: &[&str],
    extra_env: &[(&str, &str)],
) -> std::io::Result<CommandOutput> {
    let bin = fileorz_bin();
    assert!(
        bin.is_file(),
        "missing fileorz binary at {} — run cargo build -p fileorz first",
        bin.display()
    );
    let mut cmd = Command::new(&bin);
    home.apply(&mut cmd);
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    cmd.args(args);
    Ok(CommandOutput::from(cmd.output()?))
}

/// Soft UI smoke: start iced briefly; timeout exit 124 is success (still running).
pub fn ui_smoke_ok(home: &IsolatedHome, locale: &str) -> bool {
    let out = run_fileorz_capture(home, &["--ui", "--locale", locale], Some(3));
    match out {
        Ok(o) if o.status == 0 || o.status == 124 => true,
        Ok(o) => {
            eprintln!("ui smoke fail status={} err={}", o.status, o.stderr);
            false
        }
        Err(e) => {
            eprintln!("ui smoke io error: {e}");
            false
        }
    }
}
