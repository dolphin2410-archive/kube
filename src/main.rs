use std::time::Duration;
use eframe::egui::{CentralPanel, CtxRef, Image, Rgba, Vec2};
use eframe::epi::{App, Frame, IconData, Storage};
use eframe::{NativeOptions, run_native};
use eframe::egui::menu::menu;
use image::GenericImageView;

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
    let mut options = NativeOptions::default();
    let img = image::open("profile.png").unwrap();
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();
    options.icon_data = Some(IconData {
        rgba: rgba.to_vec(),
        width,
        height
    });
    options.decorated = false;


    run_native(Box::new(app), options);
}