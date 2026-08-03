//! Config / keywords roundtrip helpers used by settings editors.

#[cfg(test)]
mod tests {
    use crate::persist::{load_from_path, save_to_path};
    use crate::settings::ext_logic::set_ext;
    use crate::settings::keywords_logic::{groups_from_rows, rows_from_groups, KeywordRow};
    use crate::settings::mutex::{set_permanent, set_to_trash};
    use fileorz_core::advanced_pdf::{load_keywords, save_keywords};
    use fileorz_core::config::defaults;
    use tempfile::tempdir;

    #[test]
    fn extensions_and_autodelete_roundtrip_config() {
        let dir = tempdir().expect("temp");
        let path = dir.path().join("config.json");
        let mut cfg = defaults();
        set_ext(&mut cfg, "imagens", ".png", false);
        set_to_trash(&mut cfg.auto_delete);
        cfg.auto_delete.enabled = true;
        cfg.auto_delete.max_age_days = 30;
        save_to_path(&path, &cfg).expect("save");
        let loaded = load_from_path(&path);
        assert!(!loaded.categories["imagens"][".png"]);
        assert!(loaded.auto_delete.enabled && loaded.auto_delete.to_trash);
        assert!(!loaded.auto_delete.permanent);
        assert_eq!(loaded.auto_delete.max_age_days, 30);
        let mut again = loaded;
        set_permanent(&mut again.auto_delete);
        assert!(again.auto_delete.permanent && !again.auto_delete.to_trash);
    }

    #[test]
    fn keywords_ui_rows_roundtrip_file() {
        let dir = tempdir().expect("temp");
        let path = dir.path().join("keywords.json");
        let rows = vec![KeywordRow {
            name: "Boletos".into(),
            phrases: "boleto, fatura".into(),
        }];
        let groups = groups_from_rows(&rows);
        save_keywords(&path, &groups).expect("save");
        let loaded = load_keywords(&path).expect("load");
        let back = rows_from_groups(&loaded);
        assert_eq!(back, rows);
    }
}
