//! WordCraft Android runner.
//!
//! Native runner for Android using [`eframe`] and [`android_activity`].
//! Renders WordCraft via WGPU (Vulkan / GLES backend).

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod services;
mod shell;

#[cfg(target_os = "android")]
use android_activity::AndroidApp;

/// Android native entry point invoked by the Android activity loader.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("WordCraft")
            .with_max_level(log::LevelFilter::Info),
    );
    log::info!("Starting WordCraft on Android...");

    let storage_dir = app.internal_data_path().or_else(|| app.external_data_path());

    let inbox = wordcraft_ui_egui::Inbox::default();
    let pasted = wordcraft_ui_egui::Inbox::default();

    let region = wordcraft_ui_egui::i18n::system_region();
    let doc = wordcraft_engine::blank_document(&region);
    let mut session = wordcraft_engine::Session::new(doc);
    session.system_region = region;

    let services = services::create_services(inbox.clone(), pasted.clone(), storage_dir);
    let mut word_app = wordcraft_ui_egui::WordApp::new(session, services);
    word_app.start_screen();

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    let shell = shell::AndroidShell::new(word_app, inbox, pasted);

    if let Err(e) = eframe::run_native(
        "WordCraft",
        options,
        Box::new(move |_cc| Ok(Box::new(shell))),
    ) {
        log::error!("eframe run_native failed: {e:?}");
    }
}

/// Fallback for non-Android targets to allow checks and tests to run cleanly on desktop.
#[cfg(not(target_os = "android"))]
pub fn non_android_stub() {
    log::info!("WordCraft Android crate compiled on non-android platform.");
}
