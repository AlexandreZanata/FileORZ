//! `fileorz organize --once` single-tick command.

use crate::exit_code;
use fileorz_core::advanced_pdf::{load_keywords, KeywordGroups};
use fileorz_core::config::load_config_file;
use fileorz_core::scheduler::{run_tick, TickError};
use fileorz_linux::trash::FreedesktopTrash;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

/// Parse and run `organize --once --config <path> --folder <path> [--keywords <path>]`.
pub fn run(args: &[String]) -> ExitCode {
    if !args.iter().any(|a| a == "--once") {
        eprintln!(
            "usage: fileorz organize --once --config <path> --folder <path> [--keywords <path>]"
        );
        return ExitCode::from(exit_code::USAGE);
    }
    let Some(config_path) = flag_value(args, "--config") else {
        eprintln!("missing --config <path>");
        return ExitCode::from(exit_code::USAGE);
    };
    let Some(folder) = flag_value(args, "--folder") else {
        eprintln!("missing --folder <path>");
        return ExitCode::from(exit_code::USAGE);
    };
    let keywords_path = flag_value(args, "--keywords");

    let outcome = match load_config_file(Path::new(config_path)) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("config error: {e}");
            return ExitCode::from(exit_code::CONFIG);
        }
    };
    let keywords = match load_keywords_opt(keywords_path, outcome.config.advanced_organize) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("keywords error: {e}");
            return ExitCode::from(exit_code::CONFIG);
        }
    };
    let trash = FreedesktopTrash;
    match run_tick(
        Path::new(folder),
        &outcome.config,
        &keywords,
        SystemTime::now(),
        Some(&trash),
    ) {
        Ok(report) => {
            println!(
                "ok deletes={} pdf_moves={} pdf_skips={} ext_moves={}",
                report.deletes, report.pdf_moves, report.pdf_skips, report.ext_moves
            );
            ExitCode::from(exit_code::OK)
        }
        Err(TickError::BadFolder(p) | TickError::NotWritable(p)) => {
            eprintln!("folder error: {p}");
            ExitCode::from(exit_code::FOLDER)
        }
        Err(e) => {
            eprintln!("organize error: {e}");
            ExitCode::from(exit_code::ERROR)
        }
    }
}

fn load_keywords_opt(path: Option<&str>, advanced: bool) -> Result<KeywordGroups, String> {
    if !advanced {
        return Ok(KeywordGroups::new());
    }
    let path = path
        .map(PathBuf::from)
        .unwrap_or_else(fileorz_linux::xdg::keywords_json_path);
    if !path.is_file() {
        return Ok(KeywordGroups::new());
    }
    load_keywords(&path).map_err(|e| e.to_string())
}

fn flag_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}
