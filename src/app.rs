use eframe::{egui, epi};
use eframe::egui::{Color32, Vec2};
use eframe::epi::{Frame, Storage};
use image::GenericImageView;
use crate::tools::{Image, ImageButton};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default)]
pub struct State {
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub assets: Assets,
}

#[derive(Debug, Default)]
pub struct Assets {
    pub generals: Image,
    pub zero_hour: Image,
    pub remastered: Image,
    pub mods: Image,
    pub settings: Image,
}

impl Assets {
    pub fn load(frame: &mut epi::Frame<'_>) -> Self {
        Self {
            generals: Image::open("data/ButtonGen.png", frame),
            zero_hour: Image::open("data/ButtonZH.png", frame),
            remastered: Image::open("data/ButtonZH+.png", frame),
            mods: Image::open("data/ButtonMod.png", frame),
            settings: Image::open("data/ButtonSettings.png", frame),
        }
    }
}

impl epi::App for State {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let State { assets } = self;


        egui::CentralPanel::default().show(ctx, |ui| {
            State::style(ui);

            if ui.add(ImageButton::new(assets.remastered.texture, [500.0, 100.0])).clicked() {
                println!("Start Generals Zero Hour 1.04+");
            }
            if ui.add(ImageButton::new(assets.zero_hour.texture, [500.0, 70.0])).clicked() {
                println!("Start Generals Zero Hour");
            }
            if ui.add(ImageButton::new(assets.generals.texture, [500.0, 70.0])).clicked() {
                println!("Start Generals");
            }

            ui.horizontal(|ui| {
                if ui.add(ImageButton::new(assets.mods.texture, [410.0, 100.0])).clicked() {
                    println!("Choose Mod")
                }
                if ui.add(ImageButton::new(assets.settings.texture, [82.0, 100.0])).clicked() {
                    println!("Open Settings")
                }
            });

            egui::warn_if_debug_build(ui);
        });
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, frame: &mut Frame<'_>, storage: Option<&dyn Storage>) {
        // #[cfg(feature = "persistence")]
        // *self = epi::get_value(storage.unwrap(), epi::APP_KEY).unwrap_or_default();
        self.assets = Assets::load(frame);
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "Proof of Concept: Generals Launcher"
    }
}

impl State {
    pub fn style(ui: &mut egui::Ui) {
        let mut widgets = &mut ui.style_mut().visuals.widgets;
        widgets.inactive.bg_stroke.color = Color32::from_gray(100);
        widgets.inactive.corner_radius = 4.0;
        widgets.hovered.corner_radius = 2.0;
        widgets.active.corner_radius = 2.0;
        widgets.inactive.bg_stroke.width = 4.0;
        widgets.hovered.bg_stroke.width = 4.0;
        widgets.active.bg_stroke.width = 4.0;

        let mut spacing = ui.spacing_mut();
        spacing.item_spacing.y = 10.0;
        spacing.button_padding = Vec2::new(0.0,0.0);
    }
}