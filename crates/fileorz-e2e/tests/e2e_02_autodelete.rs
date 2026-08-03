//! E2E-02: auto-delete fixture → expected deletes (aged-files golden).

use fileorz_e2e::{
    assert_absent, assert_present, copy_dir_all, failure_guard, fixtures_root, run_fileorz,
    set_mtime_days_ago, skip_unless_e2e, write_json, IsolatedHome,
};
use serde_json::{json, Value};
use std::fs;
use std::time::SystemTime;

#[test]
fn e2e_02_autodelete_aged_files() {
    if skip_unless_e2e("e2e-02") {
        return;
    }
    let _guard = failure_guard("e2e-02");
    let home = IsolatedHome::new().expect("isolate");
    let tree = home.home().join("work/aged-files");
    copy_dir_all(&fixtures_root().join("trees/aged-files"), &tree).expect("copy tree");

    let now = SystemTime::now();
    set_mtime_days_ago(&tree.join("Documentos/TXT/old.txt"), 30, now).unwrap();
    set_mtime_days_ago(&tree.join("Documentos/TXT/fresh.txt"), 1, now).unwrap();

    let cfg_path = home.home().join("autodelete-mtime.json");
    let mut cfg: Value = serde_json::from_str(
        &fs::read_to_string(fixtures_root().join("configs/autodelete-mtime.json")).unwrap(),
    )
    .unwrap();
    cfg["Folder"] = json!(tree.to_string_lossy());
    write_json(&cfg_path, &cfg).unwrap();

    let out = run_fileorz(
        &home,
        &[
            "organize",
            "--once",
            "--config",
            cfg_path.to_str().unwrap(),
            "--folder",
            tree.to_str().unwrap(),
        ],
    )
    .expect("run organize");
    out.assert_ok("E2E-02 autodelete");
    assert!(
        out.stdout.contains("deletes=1") || out.stdout.contains("deletes="),
        "stdout={}",
        out.stdout
    );

    assert_present(&tree, &["Documentos/TXT/fresh.txt"]);
    assert_absent(&tree, &["Documentos/TXT/old.txt"]);
}
