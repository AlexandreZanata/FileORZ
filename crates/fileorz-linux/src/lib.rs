//! Linux desktop integration helpers.

pub mod trash;
pub mod xdg;

#[cfg(test)]
pub(crate) mod test_env;

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
        assert_eq!(crate_name(), "fileorz-linux");
    }
}
