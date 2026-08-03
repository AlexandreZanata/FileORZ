//! E2E-05: disable extension in settings/config → next tick honors it.

use fileorz_e2e::{
    assert_absent, assert_present, copy_dir_all, failure_guard, fixtures_root, run_fileorz,
    skip_unless_e2e, write_json, IsolatedHome,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[test]
fn e2e_05_disabled_extension_honored_on_next_tick() {
    if skip_unless_e2e("e2e-05") {
        return;
    }
    let _guard = failure_guard("e2e-05");
    let home = IsolatedHome::new().expect("isolate");
    let tree = home.home().join("work/ext-toggle");
    copy_dir_all(&fixtures_root().join("trees/tiny-mixed"), &tree).expect("copy tree");

    // .txt disabled → notes.txt must land in OUTROS (not Documentos).
    let mut categories: BTreeMap<String, Value> = BTreeMap::new();
    categories.insert("documentos".into(), json!({ ".txt": false, ".pdf": true }));
    categories.insert("imagens".into(), json!({ ".png": true }));
    categories.insert("audios".into(), json!({ ".mp3": true }));
    categories.insert("compactos".into(), json!({ ".zip": true }));

    let cfg = json!({
        "folder": tree.to_string_lossy(),
        "interval_minutes": 5,
        "autostart": false,
        "locale": "en",
        "advanced_organize": false,
        "auto_delete": { "enabled": false },
        "folder_delete": { "enabled": false },
        "categories": categories,
    });
    let cfg_path = home.home().join("ext-toggle.json");
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
    .expect("organize");
    out.assert_ok("E2E-05 organize");

    assert_present(
        &tree,
        &[
            "OUTROS/TXT/notes.txt",
            "Imagens/PNG/photo.png",
            "Audios/MP3/song.mp3",
            "Compactos/ZIP/archive.zip",
        ],
    );
    assert_absent(&tree, &["Documentos/TXT/notes.txt", "notes.txt"]);
}
