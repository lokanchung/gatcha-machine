use std::{num::NonZeroU64, sync::Arc, time::Duration};

use clap::*;
use eframe::{run_native, App, Frame as AppFrame, NativeOptions};
use egui::*;
use font_kit::source::SystemSource;
use rand::Rng;

fn main() {
    let matches = Command::new("gatch-machine")
        .version("0.1")
        .arg(Arg::new("input").action(ArgAction::Append))
        .get_matches();

    let input = matches
        .get_many::<String>("input")
        .unwrap_or_default()
        .map(|v| v.clone())
        .collect();

    let options = NativeOptions {
        initial_window_size: Some(vec2(480.0, 480.0)),
        ..Default::default()
    };

    let font_data = SystemSource::new()
        .select_by_postscript_name("MalgunGothic")
        .ok()
        .and_then(|x| x.load().ok())
        .and_then(|x| x.copy_font_data())
        .and_then(|x| Arc::into_inner(x))
        .and_then(|x| Some(FontData::from_owned(x)));

    run_native(
        "우롱차의 가차 머신",
        options,
        Box::new(|_| {
            Box::new(GatchaApp {
                font_data,
                targets: input,
                roulette: None,
            })
        }),
    )
    .unwrap();
}

#[derive(Default)]
struct Roulette {
    phase: f32, // 0.0 to 1.0
    len: usize,
    target: usize,
}

impl Roulette {
    fn current_index(&self) -> usize {
        let p = 1.0 - self.phase.clamp(0.0, 1.0);
        let p = 1.0 - p * p;
        let steps = self.len * 4 + self.target;

        (steps as f32 * p) as usize % self.len
    }
}

struct GatchaApp {
    font_data: Option<FontData>,
    targets: Vec<String>,
    roulette: Option<Roulette>,
}

impl App for GatchaApp {
    fn update(&mut self, ctx: &Context, _: &mut AppFrame) {
        // 폰트 세팅
        if let Some(font_data) = self.font_data.clone() {
            let mut fonts = FontDefinitions::default();
            fonts
                .font_data
                .insert("Malgun Gothic".to_owned(), font_data);
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "Malgun Gothic".into());

            ctx.set_fonts(fonts);
        }

        let frame = Frame::default()
            .fill(colors::BG_APP)
            .inner_margin(Margin::same(10.0));

        ctx.input(|i| {
            if i.key_pressed(Key::Space) {
                let mut rng = rand::thread_rng();
                let len = self.targets.len();
                self.roulette = Some(Roulette {
                    phase: 0.0,
                    len: len,
                    target: rng.gen_range(0..len),
                })
            }
        });

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.colored_label(
                colors::FG_TITLE,
                RichText::new("우롱차의 가차 머신").heading(),
            );

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    for (i, str) in self.targets.iter().enumerate() {
                        let mut text_color = colors::FG_TARGET;
                        let mut bg_color = colors::BG_TARGET;
                        if let Some(r) = &self.roulette {
                            if r.current_index() == i {
                                text_color = colors::FG_SELECTED_TARGET;

                                if r.phase >= 1.0 {
                                    bg_color = colors::BG_FINAL_TARGET;
                                } else {
                                    bg_color = colors::BG_SELECTED_TARGET;
                                }
                            }
                        }

                        let label = Label::new(RichText::new(str).color(text_color));

                        Frame::default()
                            .fill(bg_color)
                            .inner_margin(Margin::same(5.0))
                            .outer_margin(Margin::same(0.0))
                            .show(ui, |ui| {
                                ui.with_layout(
                                    Layout::left_to_right(Align::Min)
                                        .with_cross_justify(true)
                                        .with_main_justify(true),
                                    |ui| {
                                        ui.add(label);
                                    },
                                );
                            });
                    }
                });
            })
        });

        if let Some(r) = &mut self.roulette {
            r.phase += 0.001;
            ctx.request_repaint_after(Duration::from_millis(16));
        }
    }
}

mod colors {
    use egui::Color32;

    pub const FG_TITLE: Color32 = Color32::from_gray(250);
    pub const FG_TARGET: Color32 = Color32::from_gray(230);
    pub const FG_SELECTED_TARGET: Color32 = Color32::WHITE;
    pub const BG_TARGET: Color32 = Color32::from_gray(70);
    pub const BG_SELECTED_TARGET: Color32 = Color32::from_rgb(0x33, 0x66, 0x99);
    pub const BG_FINAL_TARGET: Color32 = Color32::from_rgb(0xAA, 0x44, 0x44);
    pub const BG_APP: Color32 = Color32::from_gray(20);
}
