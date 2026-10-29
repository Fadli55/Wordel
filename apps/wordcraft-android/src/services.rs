//! Android platform services: file storage, asynchronous document delivery, and system integration.

use std::path::PathBuf;
use wordcraft_ui_egui::{Inbox, Services};

/// Creates the Android host services bundle for [`wordcraft_ui_egui::WordApp`].
pub fn create_services(
    inbox: Inbox,
    pasted: Inbox,
    storage_dir: Option<PathBuf>,
) -> Services {
    let save_dir = storage_dir.clone();
    let download_fn: Box<dyn Fn(&str, &[u8]) -> Result<(), String>> = Box::new(move |file_name, bytes| {
        let dir = save_dir
            .as_ref()
            .cloned()
            .unwrap_or_else(|| PathBuf::from("/sdcard/Download"));

        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }

        let target_path = dir.join(file_name);
        match std::fs::write(&target_path, bytes) {
            Ok(()) => {
                log::info!("File saved successfully to: {}", target_path.display());
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to write file {}: {e}", target_path.display());
                Err(format!("Could not save file {file_name}: {e}"))
            }
        }
    });

    Services {
        inbox: Some(inbox),
        pasted: Some(pasted),
        download: Some(download_fn),
        animations_enabled: Some(Box::new(|| Some(true))),
        templates_dir: storage_dir.map(|d| d.join("Templates")),
        ..Default::default()
    }
}
