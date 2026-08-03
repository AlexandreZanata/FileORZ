//! Locale tag normalize + selection order (CLI → config → en).

/// Map common tags onto catalog directories (`en`, `pt-BR`).
#[must_use]
pub fn normalize_locale(tag: &str) -> String {
    let lower = tag.to_ascii_lowercase();
    let base = lower.split(['.', '@']).next().unwrap_or(&lower);
    if base == "en" || base.starts_with("en-") || base.starts_with("en_") {
        return "en".into();
    }
    if base == "pt" || base.starts_with("pt-") || base.starts_with("pt_") {
        return "pt-BR".into();
    }
    "en".into()
}

/// Resolve locale: `--locale` → config `locale` → default `en`.
///
/// System `LANG` / `LC_MESSAGES` are **not** used for the Linux product default
/// (always English until the user picks another language in Settings).
#[must_use]
pub fn resolve_locale(
    cli: Option<&str>,
    config: Option<&str>,
    _lang: Option<&str>,
    _lc_messages: Option<&str>,
) -> String {
    if let Some(tag) = cli.filter(|t| !t.is_empty()) {
        return normalize_locale(tag);
    }
    if let Some(tag) = config.filter(|t| !t.is_empty()) {
        return normalize_locale(tag);
    }
    "en".into()
}

/// Read process env for locale resolution (CLI / config only; env ignored).
#[must_use]
pub fn resolve_locale_from_env(cli: Option<&str>, config: Option<&str>) -> String {
    resolve_locale(
        cli,
        config,
        std::env::var("LANG").ok().as_deref(),
        std::env::var("LC_MESSAGES").ok().as_deref(),
    )
}

#[cfg(test)]
mod locale_tests {
    use super::*;

    #[test]
    fn normalize_pt_variants() {
        assert_eq!(normalize_locale("pt_BR.UTF-8"), "pt-BR");
        assert_eq!(normalize_locale("pt-br"), "pt-BR");
        assert_eq!(normalize_locale("en_US"), "en");
    }

    #[test]
    fn cli_beats_config_and_env() {
        assert_eq!(
            resolve_locale(Some("pt-BR"), Some("en"), Some("fr_FR"), None),
            "pt-BR"
        );
        assert_eq!(
            resolve_locale(None, Some("pt-BR"), Some("en_US"), None),
            "pt-BR"
        );
        // LANG must not override product default English.
        assert_eq!(resolve_locale(None, None, Some("pt_BR.UTF-8"), None), "en");
        assert_eq!(resolve_locale(None, None, None, None), "en");
    }
}
