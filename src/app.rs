use std::sync::{Arc, RwLock};

use native_dialog::{MessageDialog, MessageType};

use crate::cheat::tp;
use crate::cheat_helper::data::DataSaver;
use crate::cheat_helper::helper::Coord;

#[allow(dead_code)]
fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {
    // Example stuff:
    infinite_ammo: bool,
    tp_text: String,
    data_saver: Arc<RwLock<DataSaver>>,
    pub old_data_saver: DataSaver,
    pub local_data_saver: DataSaver,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            infinite_ammo: false,
            tp_text: "".to_owned(),
            data_saver: Arc::new(RwLock::from(DataSaver {
                infinite_ammo: false,
                tp_text: "".to_string(),
                is_running: false,
            })),
            old_data_saver: DataSaver {
                infinite_ammo: false,
                tp_text: "".to_string(),
                is_running: false,
            },
            local_data_saver: DataSaver {
                infinite_ammo: false,
                tp_text: "".to_string(),
                is_running: false,
            },
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, data_saver: Arc<RwLock<DataSaver>>) -> Self {
        Self {
            data_saver,
            ..Default::default()
        }
    }
}


impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Access the data_saver field from the struct.
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window`, or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                {
                    ui.menu_button("File", |ui| unsafe {
                        if ui.button("Quit").clicked() {
                            self.local_data_saver.is_running = false;
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Knight's cheat");
            ui.add_space(10f32);

            ui.horizontal(|ui| {
                ui.label("Infinite ammo : ");
                toggle_ui_compact(ui, &mut self.local_data_saver.infinite_ammo);
            });

            ui.horizontal(|ui| {
                ui.label("Tp to coords : ");
                ui.text_edit_singleline(&mut self.local_data_saver.tp_text);
                if ui.button("tp").clicked() {
                    let coords = Coord::parse(&self.local_data_saver.tp_text);

                    if coords.is_ok() {
                        tp(coords.unwrap());
                    } else {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_text("Error")
                            .set_text("Error while parsing tp data")
                            .show_alert()
                            .unwrap()
                    }
                }
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        if self.old_data_saver != self.local_data_saver {
            sync_data_savers(&self.local_data_saver, &self.data_saver);
            self.old_data_saver = self.local_data_saver.clone()
        }
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn sync_data_savers(data_saver_read: &DataSaver, data_saver_write: &Arc<RwLock<DataSaver>>) {
    println!("sync");
    let mut write_guard = data_saver_write.write().unwrap();
    write_guard.infinite_ammo = data_saver_read.infinite_ammo;
    write_guard.tp_text = data_saver_read.tp_text.clone();
    write_guard.is_running = data_saver_read.is_running;
}
