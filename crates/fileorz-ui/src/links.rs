//! External links for header / About actions.

/// Upstream GitHub repository.
pub const GITHUB_URL: &str = "https://github.com/ThainanViniciusKatchan/FileORZ";

/// This fork (push target).
pub const FORK_URL: &str = "https://github.com/AlexandreZanata/FileORZ";

/// Upstream changelog page.
pub const CHANGELOG_URL: &str = "https://thainanviniciuskatchan.github.io/FileORZ/changelog.html";

/// Open a URL in the desktop browser; ignore failures (no UI crash).
pub fn open_url(url: &str) {
    if let Err(err) = open::that(url) {
        eprintln!("open url failed: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_https() {
        assert!(GITHUB_URL.starts_with("https://"));
        assert!(FORK_URL.starts_with("https://"));
        assert!(CHANGELOG_URL.starts_with("https://"));
    }
}
