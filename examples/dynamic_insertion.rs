use eframe::egui;
use egui::{Rounding, Ui};
use egui_xml::load_layout;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    panel_width: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { panel_width: 250 }
    }
}

fn color_background(ui: &mut Ui, color: egui::Color32) {
    ui.painter()
        .rect_filled(ui.available_rect_before_wrap(), Rounding::same(5.0), color);
}

fn show_slider_panel(ui: &mut Ui, panel_width: &mut u32) {
    ui.add(egui::Slider::new(panel_width, 0..=500).text("Width"));
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            load_layout!(
                <Strip direction="west">
                    <Panel size="remainder" min="50.0" max="@f32::MAX">
                        show_slider_panel(ui, &mut self.panel_width);
                    </Panel>
                    <Panel size="exact" value="@self.panel_width as f32">
                        <Strip direction="north">
                            <Panel size="relative" value="0.3">
                                color_background(ui, egui::Color32::from_rgb(0, 255, 255));
                            </Panel>
                            <Panel size="remainder">
                                color_background(ui, egui::Color32::from_rgb(255, 0, 255));
                            </Panel>
                        </Strip>
                    </Panel>
                </Strip>
            );
        });
    }
}
