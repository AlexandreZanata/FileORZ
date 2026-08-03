//! E2E-01: start (UI smoke) → organize fixture folder once → golden match.

use fileorz_e2e::{
    assert_present, assert_root_remaining, copy_dir_all, failure_guard, fixtures_root, run_fileorz,
    skip_unless_e2e, ui_smoke_ok, write_json, IsolatedHome,
};
use serde_json::{json, Value};
use std::fs;

#[test]
fn e2e_01_organize_once_matches_tiny_mixed_golden() {
    if skip_unless_e2e("e2e-01") {
        return;
    }
    let _guard = failure_guard("e2e-01");
    let home = IsolatedHome::new().expect("isolate");
    let tree = home.home().join("work/tiny-mixed");
    copy_dir_all(&fixtures_root().join("trees/tiny-mixed"), &tree).expect("copy tree");

    // Soft start under Xvfb (DISPLAY set by e2e-linux.sh).
    assert!(
        ui_smoke_ok(&home, "en"),
        "UI failed to start under Xvfb (need DISPLAY + iced deps)"
    );

    let cfg_path = home.home().join("organize-basic.json");
    let mut cfg: Value = serde_json::from_str(
        &fs::read_to_string(fixtures_root().join("configs/organize-basic.json")).unwrap(),
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
    out.assert_ok("E2E-01 organize --once");
    assert!(out.stdout.contains("ext_moves="), "stdout={}", out.stdout);

    assert_present(
        &tree,
        &[
            "Documentos/TXT/notes.txt",
            "Imagens/PNG/photo.png",
            "Audios/MP3/song.mp3",
            "Compactos/ZIP/archive.zip",
            "OUTROS/XYZ/weird.xyz",
        ],
    );
    assert_root_remaining(&tree, &[".hidden"]);
}
