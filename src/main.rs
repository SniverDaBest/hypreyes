use eframe::{App, Frame, NativeOptions, egui::Context};
use egui::{pos2, CentralPanel, Color32, Rect, Stroke};
use hyprland::{data::CursorPosition, shared::HyprData};

struct HyprEyes {
    x: i64,
    y: i64,
}

impl HyprEyes {
    fn new() -> Self {
        return Self { x: 0, y: 0 };
    }

    fn update_pos(&mut self) {
        let pos = CursorPosition::get().expect("Unable to get cursor position!");
        (self.x, self.y) = (pos.x, pos.y);
    }

    fn get_pos(&self) -> (i64, i64) {
        return (self.x, self.y);
    }
}

impl App for HyprEyes {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_pos();
        ctx.request_repaint();
        CentralPanel::default().show(&ctx, |ui| {
            ui.label(format!("X: {}, Y: {}", self.get_pos().0, self.get_pos().1));
            let p = ui.painter();
            p.rect_filled(
                Rect::from_min_size(pos2(50.0, 50.0), egui::vec2(100.0, 130.0)),
                50.0,
                Color32::WHITE,
            );
            p.circle(
                Rect::from_min_size(pos2(50.0, 50.0), egui::vec2(100.0, 130.0)).center(),
                15.0,
                Color32::BLACK,
                Stroke::new(3.0, Color32::BLACK),
            );
            p.rect_filled(
                Rect::from_min_size(pos2(160.0, 50.0), egui::vec2(100.0, 130.0)),
                50.0,
                Color32::WHITE,
            );
            p.circle(
                Rect::from_min_size(pos2(160.0, 50.0), egui::vec2(100.0, 130.0)).center(),
                15.0,
                Color32::BLACK,
                Stroke::new(3.0, Color32::BLACK),
            );
        });
    }
}

fn main() {
    let native_opts = NativeOptions::default();
    eframe::run_native(
        "HyprEyes",
        native_opts,
        Box::new(|_cc| Ok(Box::new(HyprEyes::new()))),
    )
    .expect("Unable to run window!");
}
