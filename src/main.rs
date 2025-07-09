#![allow(clippy::uninlined_format_args, clippy::needless_return)]

/* TOOD:
 * Configuration
 * Make eyes strech when window resizes
 * Make window floating by default (maybe not possible)
 */

use eframe::{App, Frame, NativeOptions, egui::Context};
use egui::{CentralPanel, Color32, Rect, pos2, vec2};
use hyprland::{
    data::{Clients, CursorPosition},
    shared::HyprData,
};
use std::env;

struct HyprEyes {
    cur_x: i64,
    cur_y: i64,
    win_x: i16,
    win_y: i16,
}

impl HyprEyes {
    fn new() -> Self {
        return Self {
            cur_x: 0,
            cur_y: 0,
            win_x: 0,
            win_y: 0,
        };
    }

    fn update_cur_pos(&mut self) {
        let pos = CursorPosition::get().expect("Unable to get cursor position!");
        (self.cur_x, self.cur_y) = (pos.x, pos.y);
    }

    fn update_win_pos(&mut self) {
        for client in Clients::get().expect("Unable to get list of clients!") {
            if client.title == "HyprEyes" {
                (self.win_x, self.win_y) = client.at;
            }
        }
    }
}

impl App for HyprEyes {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_cur_pos();
        self.update_win_pos();
        ctx.request_repaint();
        CentralPanel::default().show(ctx, |ui| {
            let p = ui.painter();

            for eye_pos in [pos2(0.0, 0.0), pos2(110.0, 0.0)] {
                let eye_rect = Rect::from_min_size(eye_pos, vec2(100.0, 130.0));
                let center = eye_rect.center();

                let dist = (((self.cur_x as f32 - self.win_x as f32) - center.x)
                    * ((self.cur_x as f32 - self.win_x as f32) - center.x)
                    + ((self.cur_y as f32 - self.win_y as f32) - center.y)
                        * ((self.cur_y as f32 - self.win_y as f32) - center.y))
                    .sqrt();
                let (off_x, off_y) = if dist > 0.0 {
                    let scale = 30.0f32.min(dist) / dist;
                    (
                        ((self.cur_x as f32 - self.win_x as f32) - center.x) * scale,
                        ((self.cur_y as f32 - self.win_y as f32) - center.y) * scale,
                    )
                } else {
                    (0.0, 0.0)
                };

                p.rect_filled(eye_rect, 50.0, Color32::WHITE);

                p.circle_filled(
                    pos2(center.x + off_x, center.y + off_y),
                    15.0,
                    Color32::BLACK,
                );
            }
        });
    }
}

fn check_for_hyprland() -> Result<(), String> {
    for arg in env::args() {
        if arg == "-s" || arg == "--skip" {
            return Ok(());
        }
    }

    match env::var("DESKTOP_SESSION").expect("Unable to get desktop session environment variable! Run with the -s or --skip flag to skip this. (if you're using hyprland)").as_str() {
        "hyprland"      => return Ok(()),
        "hyprland-uwsm" => return Ok(()),
        d => {
            return Err(format!("Your desktop ({}) isn't Hyprland! HyprEyes is only for Hyprland.", d));
        },
    }
}

fn main() -> Result<(), String> {
    check_for_hyprland()?;

    let native_opts = NativeOptions::default();
    eframe::run_native(
        "HyprEyes",
        native_opts,
        Box::new(|_cc| Ok(Box::new(HyprEyes::new()))),
    )
    .expect("Unable to run window!");

    return Ok(());
}
