//! Characterization / golden fixture helpers (scaffold).
//!
//! Fixtures live under `tests/fixtures/` — see `docs/CHARACTERIZATION.md`.

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
        assert_eq!(crate_name(), "fileorz-parity");
    }
}
