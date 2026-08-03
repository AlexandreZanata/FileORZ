//! E2E-04: --tray smoke starts organizer worker without iced window.

use fileorz_e2e::{
    copy_dir_all, failure_guard, fixtures_root, run_fileorz_env, skip_unless_e2e, write_json,
    IsolatedHome,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[test]
fn e2e_04_tray_starts_worker_without_window() {
    if skip_unless_e2e("e2e-04") {
        return;
    }
    let _guard = failure_guard("e2e-04");
    let home = IsolatedHome::new().expect("isolate");
    let tree = home.home().join("work/tray-folder");
    copy_dir_all(&fixtures_root().join("trees/tiny-mixed"), &tree).expect("copy tree");

    let mut categories: BTreeMap<String, Value> = BTreeMap::new();
    let mut docs: BTreeMap<String, Value> = BTreeMap::new();
    docs.insert(".txt".into(), json!(true));
    categories.insert("documentos".into(), json!(docs));
    let cfg = json!({
        "folder": tree.to_string_lossy(),
        "interval_minutes": 60,
        "autostart": false,
        "locale": "en",
        "advanced_organize": false,
        "auto_delete": { "enabled": false },
        "folder_delete": { "enabled": false },
        "categories": categories,
    });
    write_json(&home.config_json(), &cfg).unwrap();

    let out = run_fileorz_env(
        &home,
        &["--tray", "--locale", "en"],
        &[("FILEORZ_TRAY_SMOKE", "1")],
    )
    .expect("tray smoke");
    out.assert_ok("E2E-04 tray smoke");
    assert!(
        out.stdout.contains("tray-smoke: no-window"),
        "expected no-window marker: {}",
        out.stdout
    );
    assert!(
        out.stdout.contains("tray-smoke: worker=true")
            || out.stdout.contains("tray-smoke: worker-ok"),
        "expected worker started: {}",
        out.stdout
    );
}
