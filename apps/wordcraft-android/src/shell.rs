//! Android eframe application shell.

use wordcraft_ui_egui::{Inbox, WordApp};

/// The Android shell wrapping [`WordApp`].
pub struct AndroidShell {
    pub app: WordApp,
    pub inbox: Inbox,
    pub pasted: Inbox,
}

impl AndroidShell {
    /// Creates a new Android shell instance.
    pub fn new(app: WordApp, inbox: Inbox, pasted: Inbox) -> Self {
        Self { app, inbox, pasted }
    }
}

impl eframe::App for AndroidShell {
    /// Advances one logic frame: processes commands, updates layout, and polls animations.
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.logic(ctx);
    }

    /// Forwards raw touch and input events to the application.
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.app.raw_input_hook(ctx, raw);
    }

    /// Renders the document canvas, ribbon, ruler, and dialogs.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app.ui(ui);
    }

    /// Called when the Android activity is being destroyed or paused.
    fn on_exit(&mut self) {
        log::info!("WordCraft Android shell exiting, saving recovery session...");
        self.app.session.finish_recovery();
    }
}
