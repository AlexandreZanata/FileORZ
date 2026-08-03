//! Scheduler integration tests — short interval, cooperative stop.

use crate::advanced_pdf::KeywordGroups;
use crate::config::{migrate_value, AppConfig};
use crate::scheduler::{run_tick, wait_interruptible, OrganizerHandle, OrganizerOptions};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures")
}

fn load_basic_config() -> AppConfig {
    let raw = fs::read_to_string(fixtures_root().join("configs/organize-basic.json")).unwrap();
    migrate_value(serde_json::from_str(&raw).unwrap()).unwrap()
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

#[test]
fn wait_interruptible_stops_early() {
    let stop = Arc::new(AtomicBool::new(false));
    let stopped_early = Arc::new(AtomicBool::new(false));
    let stop_w = Arc::clone(&stop);
    let flag = Arc::clone(&stopped_early);
    let handle = thread::spawn(move || {
        let continued =
            wait_interruptible(&stop_w, Duration::from_secs(30), Duration::from_millis(20));
        flag.store(!continued, Ordering::SeqCst);
    });
    thread::sleep(Duration::from_millis(40));
    stop.store(true, Ordering::SeqCst);
    handle.join().unwrap();
    assert!(stopped_early.load(Ordering::SeqCst));
}

#[test]
fn run_tick_once_moves_extensions() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("tree");
    copy_dir_all(&fixtures_root().join("trees/tiny-mixed"), &root).unwrap();
    let cfg = load_basic_config();
    let report = run_tick(&root, &cfg, &KeywordGroups::new(), SystemTime::now(), None).unwrap();
    assert!(report.ext_moves >= 4);
    assert!(root.join("Documentos/TXT/notes.txt").is_file());
}

#[test]
fn drop_file_mid_run_then_stop_cleanly() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("folder");
    fs::create_dir_all(&root).unwrap();
    let cfg = load_basic_config();
    let handle = OrganizerHandle::start(OrganizerOptions {
        root: root.clone(),
        config: cfg,
        keywords: KeywordGroups::new(),
        interval: Duration::from_millis(40),
        trash: None,
    });
    thread::sleep(Duration::from_millis(30));
    fs::write(root.join("notes.txt"), b"hello").unwrap();
    let dest = root.join("Documentos/TXT/notes.txt");
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline && !dest.is_file() {
        thread::sleep(Duration::from_millis(20));
    }
    assert!(dest.is_file(), "organizer should move dropped file");
    handle
        .stop(Duration::from_secs(2))
        .expect("cooperative stop must join without kill");
}
