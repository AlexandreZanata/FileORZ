//! Optional UI smoke capture (`FILEORZ_UI_SMOKE=<ppm-path>`).

use crate::message::Message;
use iced::window;
use iced::Task;
use std::path::{Path, PathBuf};

/// Capture oldest window into a [`Message::SmokeSave`].
pub fn capture(path: PathBuf) -> Task<Message> {
    window::get_oldest().then(move |id| match id {
        Some(id) => {
            let path = path.clone();
            window::screenshot(id).map(move |shot| Message::SmokeSave {
                path: path.clone(),
                bytes: shot.bytes.to_vec(),
                width: shot.size.width,
                height: shot.size.height,
            })
        }
        None => Task::done(Message::Quit),
    })
}

/// Write RGBA screenshot as uncompressed PPM (no extra image deps).
pub fn write_ppm(path: &Path, width: u32, height: u32, rgba: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut out = format!("P6\n{width} {height}\n255\n").into_bytes();
    for chunk in rgba.chunks_exact(4) {
        out.extend_from_slice(&chunk[..3]);
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn write_ppm_roundtrip_header() {
        let dir = tempdir().expect("temp");
        let path = dir.path().join("t.ppm");
        let rgba = [10_u8, 20, 30, 255, 40, 50, 60, 255];
        write_ppm(&path, 2, 1, &rgba).expect("write");
        let raw = std::fs::read(&path).expect("read");
        assert!(raw.starts_with(b"P6\n2 1\n255\n"));
        assert_eq!(&raw[raw.len() - 6..], &[10, 20, 30, 40, 50, 60]);
    }
}
