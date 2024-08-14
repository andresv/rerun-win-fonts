use crate::rrlog;
use eframe::egui;
use re_viewer::external::eframe;

const BUTTON_SIZE: egui::Vec2 = egui::vec2(85., 25.);

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    rerun_app: re_viewer::App,
    rr: rerun::RecordingStream,
    counter: f64,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, rerun_app: re_viewer::App) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            rerun_app,
            rr: rrlog::recording_stream().unwrap(),
            counter: 0.0,
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.counter += 1.0;
        self.rr
            .log(rrlog::CURRENT_I54V, &rerun::Scalar::new(self.counter))
            .unwrap();

        //ctx.request_repaint();

        egui::SidePanel::right("left_panel")
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    ui.add_space(5.);
                    ui.label("WINRERUN");

                    egui::ScrollArea::horizontal()
                        .id_source("scroll_top")
                        .show(ui, |ui| {
                            // GET ULEDWARE
                            ui.horizontal(|ui| if button_clicked(ui, "RUN", true) {});
                        });

                    // LOGS
                    ui.add_space(10.);
                    ui.label("CONTROL");
                    let mut ctrl = 100;
                    ui.add(egui::Slider::new(&mut ctrl, 0..=1000).text("LED1"));
                    ui.add(egui::Slider::new(&mut ctrl, 0..=1000).text("LED2"));
                    ui.add(egui::Slider::new(&mut ctrl, 0..=1000).text("LED3"));
                    ui.add(egui::Slider::new(&mut ctrl, 0..=1000).text("LED4"));

                    ui.set_width(ui.available_width());

                    // Fill with empty space.
                    //ui.allocate_space(ui.available_size());
                });
            });

        // Show the Rerun Viewer in the remaining space.
        self.rerun_app.update(ctx, frame);
    }
}

pub fn button_clicked(ui: &mut egui::Ui, label: &str, started: bool) -> bool {
    let mut clicked = false;
    ui.scope(|ui| {
        ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
        let button = if started {
            egui::Button::new(label).fill(egui::Color32::GREEN)
        } else {
            egui::Button::new(label).fill(egui::Color32::GRAY)
        };

        if ui.add_sized(BUTTON_SIZE, button).clicked() {
            clicked = true;
        }
    });

    clicked
}
