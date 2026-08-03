//! FileORZ binary — CLI entry (scaffold + locale demo).

use fileorz_i18n::{resolve_locale_from_env, Localization};
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
           --locale <TAG>   Locale override (en, pt-BR)\n\
           --demo-i18n      Print sample strings for resolved locale\n",
        env!("CARGO_PKG_VERSION")
    );
}

fn print_version() {
    println!("fileorz {}", env!("CARGO_PKG_VERSION"));
}

fn locale_arg(args: &[String]) -> Option<&str> {
    args.iter()
        .position(|a| a == "--locale")
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
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
    if args.iter().any(|a| a == "--demo-i18n") {
        return demo_i18n(locale_arg(&args));
    }
    println!(
        "fileorz {} — scaffold ({})",
        env!("CARGO_PKG_VERSION"),
        fileorz_core::crate_name()
    );
    ExitCode::SUCCESS
}

fn demo_i18n(cli_locale: Option<&str>) -> ExitCode {
    let tag = resolve_locale_from_env(cli_locale, None);
    match Localization::embed(&tag) {
        Ok(loc) => {
            println!("locale={}", loc.locale());
            println!("app-title={}", loc.message("app-title"));
            println!("main-btn-start={}", loc.message("main-btn-start"));
            println!(
                "error-folder-missing={}",
                loc.message("error-folder-missing")
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("i18n error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_env_is_semver_like() {
        let v = env!("CARGO_PKG_VERSION");
        assert!(v.split('.').count() >= 2);
    }
}
