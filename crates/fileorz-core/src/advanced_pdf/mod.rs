//! Advanced PDF keyword organize (B-14) — extract, match, plan, apply.

mod apply;
mod extract;
mod keywords;
mod plan;

pub use apply::{apply_pdf_actions, ApplyError};
pub use extract::{last_page_haystack, ExtractError};
pub use keywords::{find_first_group, load_keywords, KeywordGroups, KeywordsError};
pub use plan::{plan_pdf_actions, PdfAction, PlanError, SkipReason};

#[cfg(test)]
mod tests;
