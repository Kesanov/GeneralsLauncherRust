use eframe::{egui, epi};
use eframe::egui::Color32;
use image;
use image::GenericImageView;
use itertools::Itertools;
use eframe::egui::{Sense, widgets, TextureId, Vec2, WidgetInfo, WidgetType, Ui, Widget, Response};
use std::fs;

pub fn default<T: Default>() -> T {
    Default::default()
}


////////// IMAGE LOADING //////////

/// Immediate mode texture manager that supports at most one texture at the time :)
#[derive(Debug, Default)]
pub struct Image {
    pub size: (usize, usize),
    pub data: Vec<Color32>,
    pub texture: egui::TextureId,
}

impl Image {
    pub fn open(file: &str, frame: &mut epi::Frame<'_>) -> Self {
        if let Some(image) = image::open(file).ok() {
            let buffer = image.to_rgba8();
            let size   = (image.width() as usize, image.height() as usize);
            let data   = buffer.into_vec()
                .chunks(4)
                .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect_vec();

            let texture = frame.tex_allocator().alloc_srgba_premultiplied(size, &data);
            Self { size, data, texture }
        } else {
            Self::default()
        }
    }
}



////////// BUGFIXES //////////


/// A clickable image within a frame.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
#[derive(Clone, Debug)]
pub struct ImageButton {
    image: widgets::Image,
    sense: Sense,
    frame: bool,
    selected: bool,
}

impl ImageButton {
    pub fn new(texture_id: TextureId, size: impl Into<Vec2>) -> Self {
        Self {
            image: widgets::Image::new(texture_id, size),
            sense: Sense::click(),
            frame: true,
            selected: false,
        }
    }
}

impl Widget for ImageButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { image, sense, frame, selected, } = self;

        let button_padding = ui.spacing().button_padding;
        let size = image.size() + 2.0*button_padding;
        let (rect, response) = ui.allocate_exact_size(size, sense);
        response.widget_info(|| WidgetInfo::new(WidgetType::ImageButton));

        if ui.clip_rect().intersects(rect) {
            let image_rect = ui
                .layout()
                .align_size_within_rect(image.size(), rect.shrink2(button_padding));
            image.paint_at(ui, image_rect);
            let visuals = ui.style().interact(&response);

            if selected {
                let selection = ui.visuals().selection;
                ui.painter()
                    .rect_stroke(rect, visuals.corner_radius, selection.stroke);
            } else if frame {
                ui.painter().rect_stroke(
                    rect,
                    visuals.corner_radius,
                    visuals.bg_stroke,
                );
            }
        }
        response
    }
}
