//! External links for header / About actions.

use std::io;
use std::process::{Command, Stdio};
use std::thread;

/// Upstream GitHub repository (original FileORZ).
pub const UPSTREAM_URL: &str = "https://github.com/ThainanViniciusKatchan/FileORZ";

/// This fork (Linux product / push target).
pub const FORK_URL: &str = "https://github.com/AlexandreZanata/FileORZ";

/// Header **GitHub** button → this fork.
pub const GITHUB_URL: &str = FORK_URL;

/// Header **Changelog** button → Linux v1 release notes on GitHub.
pub const CHANGELOG_URL: &str =
    "https://github.com/AlexandreZanata/FileORZ/releases/tag/linux-v1.0.0";

/// About → third-party notices file on the fork.
pub const NOTICES_URL: &str =
    "https://github.com/AlexandreZanata/FileORZ/blob/main/THIRD_PARTY_NOTICES.md";

/// Open a URL in the desktop browser (non-blocking).
pub fn open_url(url: &str) {
    let url = url.to_string();
    thread::spawn(move || {
        if let Err(err) = open_in_browser(&url) {
            eprintln!("open url failed ({url}): {err}");
        }
    });
}

/// Prefer `xdg-open` so the real default browser gets focus (not IDE helpers).
fn open_in_browser(url: &str) -> io::Result<()> {
    let mut cmd = Command::new("xdg-open");
    cmd.arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // Cursor/VS Code often set BROWSER to a helper that "opens" without navigating.
    if browser_env_is_ide_helper() {
        cmd.env_remove("BROWSER");
    }
    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(_) => open::that_detached(url),
    }
}

fn browser_env_is_ide_helper() -> bool {
    let Ok(browser) = std::env::var("BROWSER") else {
        return false;
    };
    let lower = browser.to_ascii_lowercase();
    lower.contains("cursor")
        || lower.contains("code")
        || lower.contains("electron")
        || lower.contains("vscode")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_https() {
        assert!(UPSTREAM_URL.starts_with("https://"));
        assert!(FORK_URL.starts_with("https://"));
        assert!(GITHUB_URL.starts_with("https://"));
        assert!(CHANGELOG_URL.starts_with("https://"));
        assert!(NOTICES_URL.starts_with("https://"));
        assert_eq!(GITHUB_URL, FORK_URL);
        assert!(CHANGELOG_URL.contains("linux-v1.0.0"));
    }
}
