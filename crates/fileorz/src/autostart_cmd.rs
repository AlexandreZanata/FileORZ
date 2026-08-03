//! `fileorz autostart` — enable / disable / status / print dry-run.

use crate::exit_code;
use fileorz_linux::autostart;
use std::process::ExitCode;

pub fn run(args: &[String]) -> ExitCode {
    let Some(cmd) = args.first().map(String::as_str) else {
        usage();
        return ExitCode::from(exit_code::USAGE);
    };
    match cmd {
        "enable" => match autostart::enable() {
            Ok(path) => {
                println!("enabled {}", path.display());
                ExitCode::from(exit_code::OK)
            }
            Err(e) => {
                eprintln!("autostart enable failed: {e}");
                ExitCode::from(exit_code::ERROR)
            }
        },
        "disable" => match autostart::disable() {
            Ok(()) => {
                println!("disabled {}", autostart::path().display());
                ExitCode::from(exit_code::OK)
            }
            Err(e) => {
                eprintln!("autostart disable failed: {e}");
                ExitCode::from(exit_code::ERROR)
            }
        },
        "status" => {
            if autostart::is_enabled() {
                println!("enabled {}", autostart::path().display());
            } else {
                println!("disabled {}", autostart::path().display());
            }
            ExitCode::from(exit_code::OK)
        }
        "print" => {
            print!("{}", autostart::desktop_entry("fileorz --tray"));
            println!("# path: {}", autostart::path().display());
            ExitCode::from(exit_code::OK)
        }
        _ => {
            usage();
            ExitCode::from(exit_code::USAGE)
        }
    }
}

fn usage() {
    eprintln!("usage: fileorz autostart <enable|disable|status|print>");
}
