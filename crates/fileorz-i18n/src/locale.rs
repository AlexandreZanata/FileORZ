//! Locale tag normalize + selection order (CLI → config → env → en).

/// Map common tags onto catalog directories (`en`, `pt-BR`).
#[must_use]
pub fn normalize_locale(tag: &str) -> String {
    let cleaned = tag.split('.').next().unwrap_or(tag).trim();
    let lower = cleaned.replace('_', "-").to_ascii_lowercase();
    if lower == "en" || lower.starts_with("en-") {
        return "en".into();
    }
    if lower == "pt" || lower == "pt-br" || lower.starts_with("pt-br") {
        return "pt-BR".into();
    }
    cleaned.replace('_', "-")
}

/// Resolve locale: `--locale` → config → `LANG`/`LC_MESSAGES` → `en`.
#[must_use]
pub fn resolve_locale(
    cli: Option<&str>,
    config: Option<&str>,
    lang: Option<&str>,
    lc_messages: Option<&str>,
) -> String {
    if let Some(tag) = first_nonempty(cli) {
        return normalize_locale(tag);
    }
    if let Some(tag) = first_nonempty(config) {
        return normalize_locale(tag);
    }
    if let Some(tag) = first_nonempty(lang).or_else(|| first_nonempty(lc_messages)) {
        return normalize_locale(tag);
    }
    "en".into()
}

/// Read process env for locale resolution (`LANG`, then `LC_MESSAGES`).
#[must_use]
pub fn resolve_locale_from_env(cli: Option<&str>, config: Option<&str>) -> String {
    resolve_locale(
        cli,
        config,
        std::env::var("LANG").ok().as_deref(),
        std::env::var("LC_MESSAGES").ok().as_deref(),
    )
}

fn first_nonempty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|s| !s.is_empty())
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
        assert_eq!(
            resolve_locale(None, None, Some("pt_BR.UTF-8"), None),
            "pt-BR"
        );
        assert_eq!(resolve_locale(None, None, None, None), "en");
    }
}
