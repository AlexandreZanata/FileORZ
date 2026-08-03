//! Extensions editor helpers (toggle / bulk).

use fileorz_core::config::{AppConfig, ExtMap};

/// Set one extension toggle inside a category.
pub fn set_ext(config: &mut AppConfig, category: &str, ext: &str, enabled: bool) {
    if let Some(map) = config.categories.get_mut(category) {
        map.insert(ext.to_string(), enabled);
    }
}

/// Enable or disable every extension in a category.
pub fn set_category_all(config: &mut AppConfig, category: &str, enabled: bool) {
    if let Some(map) = config.categories.get_mut(category) {
        for value in map.values_mut() {
            *value = enabled;
        }
    }
}

/// Sorted category ids for stable UI order.
#[must_use]
pub fn category_ids(config: &AppConfig) -> Vec<String> {
    config.categories.keys().cloned().collect()
}

/// Sorted extension keys for a category.
#[must_use]
pub fn ext_keys(map: &ExtMap) -> Vec<String> {
    map.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fileorz_core::config::defaults;
    use fileorz_core::organize::plan_moves;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn toggle_and_bulk_roundtrip_in_config() {
        let mut cfg = defaults();
        set_ext(&mut cfg, "documentos", ".txt", false);
        assert_eq!(
            cfg.categories["documentos"].get(".txt").copied(),
            Some(false)
        );
        set_category_all(&mut cfg, "documentos", true);
        assert!(cfg.categories["documentos"].values().all(|v| *v));
        set_category_all(&mut cfg, "documentos", false);
        assert!(cfg.categories["documentos"].values().all(|v| !*v));
    }

    #[test]
    fn disabling_ext_changes_organize_destination() {
        let dir = tempdir().expect("temp");
        fs::write(dir.path().join("notes.txt"), b"hi").expect("write");
        let mut cfg = defaults();
        set_ext(&mut cfg, "documentos", ".txt", false);
        let plan = plan_moves(dir.path(), &cfg).expect("plan");
        let mv = plan
            .iter()
            .find(|m| m.from_name == "notes.txt")
            .expect("planned");
        assert!(
            mv.to_rel.starts_with("OUTROS/"),
            "disabled .txt falls through to OUTROS, got {}",
            mv.to_rel
        );
        set_ext(&mut cfg, "documentos", ".txt", true);
        let plan2 = plan_moves(dir.path(), &cfg).expect("plan2");
        let mv2 = plan2
            .iter()
            .find(|m| m.from_name == "notes.txt")
            .expect("planned2");
        assert!(
            mv2.to_rel.starts_with("Documentos/"),
            "re-enabled .txt goes to Documentos, got {}",
            mv2.to_rel
        );
    }
}
