//! PDF text extract — last successfully read page (Alg.py parity quirk).

use lopdf::Document;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("pdf load: {0}")]
    Load(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Uppercased text from the last page that extracts successfully.
///
/// Upstream overwrites `texto_completo` each page; empty extract still
/// overwrites. Failures on a page leave the previous haystack.
pub fn last_page_haystack(path: &Path) -> Result<String, ExtractError> {
    let doc = Document::load(path).map_err(|e| ExtractError::Load(e.to_string()))?;
    let mut pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    pages.sort_unstable();
    let mut haystack = String::new();
    for page in pages {
        match doc.extract_text(&[page]) {
            Ok(text) => haystack = text.to_uppercase(),
            Err(_) => continue,
        }
    }
    Ok(haystack)
}
