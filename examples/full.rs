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
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp))),
    )
}

struct MyApp;

fn color_background(ui: &mut Ui, color: egui::Color32) {
    ui.painter()
        .rect_filled(ui.available_rect_before_wrap(), Rounding::same(5.0), color);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let vertical_gap_symetric = 1.25;

            load_layout!(
                <Strip direction="north" gap="@vertical_gap_symetric">
                    <Strip direction="north" gap="@vertical_gap_symetric" size="relative" value="0.5">
                        <Panel size="relative" value="0.4">
                            <Strip direction="west">
                                <Panel size="exact" value="250.0">
                                    color_background(ui, egui::Color32::from_rgb(255, 255, 0));
                                </Panel>
                                <Panel size="remainder">
                                    color_background(ui, egui::Color32::from_rgb(255, 0, 0));
                                </Panel>
                            </Strip>
                        </Panel>
                        <Panel size="remainder">
                            <Strip direction="west">
                                <Panel size="relative" value="0.3">
                                    color_background(ui, egui::Color32::from_rgb(0, 0, 255));
                                </Panel>
                                <Panel size="remainder">
                                    <Strip direction="north" gap="@vertical_gap_symetric">
                                        <Panel size="relative" value="0.3">
                                            color_background(ui, egui::Color32::from_rgb(0, 255, 255));
                                        </Panel>
                                        <Panel size="remainder">
                                            color_background(ui, egui::Color32::from_rgb(255, 0, 255));
                                        </Panel>
                                    </Strip>
                                </Panel>
                            </Strip>
                        </Panel>
                    </Strip>
                    <Strip direction="north" gap="@vertical_gap_symetric" size="remainder">
                        <Panel size="relative" value="0.4">
                            <Strip direction="west">
                                <Panel size="exact" value="250.0">
                                    color_background(ui, egui::Color32::from_rgb(255, 255, 0));
                                </Panel>
                                <Panel size="remainder">
                                    color_background(ui, egui::Color32::from_rgb(255, 0, 0));
                                </Panel>
                            </Strip>
                        </Panel>
                        <Panel size="remainder">
                            <Strip direction="west">
                                <Panel size="relative" value="0.3">
                                    color_background(ui, egui::Color32::from_rgb(0, 0, 255));
                                </Panel>
                                <Panel size="remainder">
                                    <Strip direction="north" gap="@vertical_gap_symetric">
                                        <Panel size="relative" value="0.3">
                                            color_background(ui, egui::Color32::from_rgb(0, 255, 255));
                                        </Panel>
                                        <Panel size="remainder">
                                            color_background(ui, egui::Color32::from_rgb(255, 0, 255));
                                        </Panel>
                                    </Strip>
                                </Panel>
                            </Strip>
                        </Panel>
                    </Strip>
                </Strip>
            );
        });
    }
}
