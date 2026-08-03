//! Supported UI locales for the language picker.

/// Locales shipped in `fileorz-i18n` catalogs.
pub const LOCALE_CHOICES: &[&str] = &["en", "pt-BR"];

/// Normalize picker selection into a catalog tag.
#[must_use]
pub fn normalize_pick(tag: &str) -> &'static str {
    match tag {
        "pt-BR" | "pt_BR" | "pt" => "pt-BR",
        _ => "en",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_normalize() {
        assert_eq!(normalize_pick("pt-BR"), "pt-BR");
        assert_eq!(normalize_pick("en"), "en");
        assert_eq!(normalize_pick("de"), "en");
    }
}
