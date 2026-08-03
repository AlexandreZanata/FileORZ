//! Domain logic for organize / auto-delete / advanced PDF.

pub mod advanced_pdf;
pub mod autodelete;
pub mod config;
pub mod organize;

/// Crate package name (smoke helper for workspace wiring).
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_matches() {
        assert_eq!(crate_name(), "fileorz-core");
    }
}
