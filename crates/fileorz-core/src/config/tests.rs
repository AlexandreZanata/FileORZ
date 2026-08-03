//! Config migration and load/save tests.

use super::{
    atomic_write_json, defaults, legacy_backup_path, load_config_file, looks_legacy, migrate_value,
    parse_config_json, AppConfig,
};
use std::fs;
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/configs")
        .join(name)
}

#[test]
fn defaults_parse_and_have_categories() {
    let cfg = defaults();
    assert_eq!(cfg.interval_minutes, 5);
    assert!(cfg.categories.contains_key("documentos"));
    assert!(cfg.folder.is_none());
}

#[test]
fn migrate_legacy_pt_fixture() {
    let raw = fs::read_to_string(fixture("legacy-pt.json")).unwrap();
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert!(looks_legacy(&value));
    let cfg = migrate_value(value).unwrap();
    assert_eq!(cfg.folder.as_deref(), Some("/tmp/fileorz-fixture-tiny"));
    assert_eq!(cfg.interval_minutes, 5);
    assert!(cfg.auto_delete.to_trash);
    assert!(!cfg.auto_delete.permanent);
    assert!(cfg.auto_delete.by_modified);
    assert_eq!(cfg.auto_delete.max_age_days, 15);
    assert!(cfg.categories["imagens"][".jpg"]);
    assert!(!cfg.categories["imagens"][".png"]);
}

#[test]
fn parse_stable_v1_fixture() {
    let bytes = fs::read(fixture("stable-v1.json")).unwrap();
    let cfg = parse_config_json(&bytes).unwrap();
    assert_eq!(cfg.locale, "en");
    assert_eq!(cfg.interval_minutes, 5);
    assert!(cfg.categories.contains_key("documentos"));
}

#[test]
fn legacy_load_writes_backup_and_stable_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("config.json");
    fs::copy(fixture("legacy-pt.json"), &path).unwrap();
    let first = load_config_file(&path).unwrap();
    assert!(first.migrated);
    let bak = legacy_backup_path(&path);
    assert!(bak.is_file());
    assert!(!looks_legacy(
        &serde_json::from_slice(&fs::read(&path).unwrap()).unwrap()
    ));
    let second = load_config_file(&path).unwrap();
    assert!(!second.migrated);
    assert_eq!(first.config, second.config);
    let bak_bytes = fs::read(&bak).unwrap();
    assert!(looks_legacy(&serde_json::from_slice(&bak_bytes).unwrap()));
}

#[test]
fn atomic_write_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("config.json");
    let cfg = AppConfig {
        folder: Some("/tmp/x".into()),
        ..AppConfig::default()
    };
    atomic_write_json(&path, &cfg).unwrap();
    let loaded = load_config_file(&path).unwrap();
    assert!(!loaded.migrated);
    assert_eq!(loaded.config.folder.as_deref(), Some("/tmp/x"));
}
