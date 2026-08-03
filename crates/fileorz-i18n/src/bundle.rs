//! Fluent bundle build + message lookup with en fallback.

use crate::catalog::embedded_ftl;
use fluent::{FluentBundle, FluentResource};
use std::fs;
use std::path::Path;
use thiserror::Error;
use unic_langid::LanguageIdentifier;

#[derive(Debug, Error)]
pub enum I18nError {
    #[error("parse ftl: {0}")]
    Parse(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported locale catalog: {0}")]
    Unsupported(String),
}

/// Loaded primary (+ optional English fallback) Fluent bundles.
pub struct Localization {
    locale: String,
    primary: FluentBundle<FluentResource>,
    english: Option<FluentBundle<FluentResource>>,
}

impl Localization {
    /// Load embedded catalogs for `locale` (falls back to `en` resources).
    pub fn embed(locale: &str) -> Result<Self, I18nError> {
        let primary_tag = if embedded_ftl(locale).is_some() {
            locale
        } else {
            "en"
        };
        let primary = bundle_from_source(primary_tag, embedded_ftl(primary_tag).unwrap())?;
        let english = if primary_tag == "en" {
            None
        } else {
            Some(bundle_from_source("en", embedded_ftl("en").unwrap())?)
        };
        Ok(Self {
            locale: primary_tag.to_string(),
            primary,
            english,
        })
    }

    /// Load `dir/{locale}/*.ftl` from disk (dev / tests).
    pub fn from_dir(dir: &Path, locale: &str) -> Result<Self, I18nError> {
        let primary_tag = if dir.join(locale).is_dir() {
            locale
        } else if dir.join("en").is_dir() {
            "en"
        } else {
            return Err(I18nError::Unsupported(locale.into()));
        };
        let primary = bundle_from_source(primary_tag, &read_locale_dir(&dir.join(primary_tag))?)?;
        let english = if primary_tag == "en" {
            None
        } else {
            Some(bundle_from_source(
                "en",
                &read_locale_dir(&dir.join("en"))?,
            )?)
        };
        Ok(Self {
            locale: primary_tag.to_string(),
            primary,
            english,
        })
    }

    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Lookup message: primary → en → message-id literal.
    #[must_use]
    pub fn message(&self, id: &str) -> String {
        if let Some(text) = format_id(&self.primary, id) {
            return text;
        }
        if let Some(en) = &self.english {
            if let Some(text) = format_id(en, id) {
                return text;
            }
        }
        id.to_string()
    }
}

fn bundle_from_source(
    locale: &str,
    source: &str,
) -> Result<FluentBundle<FluentResource>, I18nError> {
    let lang: LanguageIdentifier = locale
        .parse()
        .unwrap_or_else(|_| "en".parse().expect("en is valid"));
    let resource = FluentResource::try_new(source.to_string())
        .map_err(|(_, errs)| I18nError::Parse(format!("{errs:?}")))?;
    let mut bundle = FluentBundle::new(vec![lang]);
    bundle
        .add_resource(resource)
        .map_err(|e| I18nError::Parse(format!("{e:?}")))?;
    // UI labels are plain text; disable Unicode isolates for simple equality tests.
    bundle.set_use_isolating(false);
    Ok(bundle)
}

fn format_id(bundle: &FluentBundle<FluentResource>, id: &str) -> Option<String> {
    let msg = bundle.get_message(id)?;
    let pattern = msg.value()?;
    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, None, &mut errors);
    if !errors.is_empty() {
        return None;
    }
    Some(value.into_owned())
}

fn read_locale_dir(dir: &Path) -> Result<String, I18nError> {
    let mut parts = Vec::new();
    for name in crate::catalog::FILES {
        let path = dir.join(name);
        if path.is_file() {
            parts.push(fs::read_to_string(path)?);
        }
    }
    if parts.is_empty() {
        return Err(I18nError::Unsupported(dir.display().to_string()));
    }
    Ok(parts.join("\n"))
}
