//! External links for header / About actions.

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
        assert!(UPSTREAM_URL.starts_with("https://"));
        assert!(FORK_URL.starts_with("https://"));
        assert!(GITHUB_URL.starts_with("https://"));
        assert!(CHANGELOG_URL.starts_with("https://"));
        assert!(NOTICES_URL.starts_with("https://"));
        assert_eq!(GITHUB_URL, FORK_URL);
        assert!(CHANGELOG_URL.contains("linux-v1.0.0"));
    }
}
