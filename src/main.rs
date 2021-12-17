use std::time::Duration;
use eframe::egui::{CentralPanel, CtxRef, Rgba, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};

pub mod env;

pub struct MainApplication;

impl App for MainApplication {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("HI");
            if ui.button("CLICK").clicked() {
                println!("HA");
            }
        });
    }

    fn setup(&mut self, _ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {

    }

    fn name(&self) -> &str {
        "NAME"
    }
}

fn main() {
    let app = MainApplication;
    run_native(Box::new(app), NativeOptions::default());
}