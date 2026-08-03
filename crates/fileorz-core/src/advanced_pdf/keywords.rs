//! Keyword groups JSON (`keywords.json`) — insertion order preserved.

use indexmap::IndexMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Group name → phrase list (JSON object order = match precedence).
pub type KeywordGroups = IndexMap<String, Vec<String>>;

#[derive(Debug, Error)]
pub enum KeywordsError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

/// Load keyword groups from disk (XDG `keywords.json` shape).
pub fn load_keywords(path: &Path) -> Result<KeywordGroups, KeywordsError> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

/// Atomic write of keyword groups (temp + rename).
pub fn save_keywords(path: &Path, groups: &KeywordGroups) -> Result<(), KeywordsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    {
        let body = serde_json::to_vec_pretty(groups)?;
        let mut file = fs::File::create(&tmp)?;
        use std::io::Write;
        file.write_all(&body)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

/// First group whose phrase is an uppercase substring of `haystack`.
#[must_use]
pub fn find_first_group<'a>(haystack: &str, groups: &'a KeywordGroups) -> Option<&'a str> {
    for (group, phrases) in groups {
        for phrase in phrases {
            if haystack.contains(&phrase.to_uppercase()) {
                return Some(group.as_str());
            }
        }
    }
    None
}

#[cfg(test)]
mod match_tests {
    use super::*;

    #[test]
    fn first_group_wins_over_later() {
        let mut groups = KeywordGroups::new();
        groups.insert("A".into(), vec!["FOO".into()]);
        groups.insert("B".into(), vec!["FOO".into()]);
        assert_eq!(find_first_group("xx FOO yy", &groups), Some("A"));
    }

    #[test]
    fn case_insensitive_substring() {
        let mut groups = KeywordGroups::new();
        groups.insert("Nota Fiscal".into(), vec!["Nota Fiscal".into()]);
        assert_eq!(
            find_first_group("NOTA FISCAL\n", &groups),
            Some("Nota Fiscal")
        );
    }

    #[test]
    fn save_keywords_roundtrip() {
        let dir = tempfile::tempdir().expect("temp");
        let path = dir.path().join("keywords.json");
        let mut groups = KeywordGroups::new();
        groups.insert("Invoices".into(), vec!["invoice".into(), "receipt".into()]);
        save_keywords(&path, &groups).expect("save");
        let loaded = load_keywords(&path).expect("load");
        assert_eq!(loaded, groups);
    }
}
