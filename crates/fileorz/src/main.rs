//! FileORZ binary — CLI entry (scaffold).

use std::env;
use std::process::ExitCode;

fn print_help() {
    println!(
        "fileorz {}\n\n\
         Usage: fileorz [OPTIONS]\n\n\
         Options:\n\
           -h, --help       Print help\n\
           -V, --version    Print version\n\
           --tray           Start hidden in tray (reserved)\n\
           --locale <TAG>   Locale override (reserved)\n",
        env!("CARGO_PKG_VERSION")
    );
}

fn print_version() {
    println!("fileorz {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        print_version();
        return ExitCode::SUCCESS;
    }
    println!(
        "fileorz {} — scaffold ({})",
        env!("CARGO_PKG_VERSION"),
        fileorz_core::crate_name()
    );
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_env_is_semver_like() {
        let v = env!("CARGO_PKG_VERSION");
        assert!(v.split('.').count() >= 2);
    }
}
