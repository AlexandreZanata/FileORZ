//! Plan PDF keyword moves (no filesystem writes beyond reads).

use crate::advanced_pdf::extract::last_page_haystack;
use crate::advanced_pdf::keywords::{find_first_group, KeywordGroups};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    DestinationExists,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfAction {
    Move {
        from_name: String,
        to_rel: String,
    },
    Skip {
        from_name: String,
        to_rel: String,
        reason: SkipReason,
    },
}

#[derive(Debug, Error)]
pub enum PlanError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("extract {0}: {1}")]
    Extract(String, String),
}

/// Plan actions for root `*.pdf` files (sorted names; first keyword match wins).
pub fn plan_pdf_actions(
    root: &Path,
    keywords: &KeywordGroups,
) -> Result<Vec<PdfAction>, PlanError> {
    let mut actions = Vec::new();
    for name in list_root_pdfs(root)? {
        let haystack = last_page_haystack(&root.join(&name))
            .map_err(|e| PlanError::Extract(name.clone(), e.to_string()))?;
        let Some(group) = find_first_group(&haystack, keywords) else {
            continue;
        };
        let to_rel = format!("{group}/{name}");
        let dest = root.join(&to_rel);
        if dest.exists() {
            actions.push(PdfAction::Skip {
                from_name: name,
                to_rel,
                reason: SkipReason::DestinationExists,
            });
        } else {
            actions.push(PdfAction::Move {
                from_name: name,
                to_rel,
            });
        }
    }
    Ok(actions)
}

fn list_root_pdfs(root: &Path) -> Result<Vec<String>, PlanError> {
    let mut names = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".pdf") {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}
