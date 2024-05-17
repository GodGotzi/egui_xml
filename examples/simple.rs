use eframe::egui;
use egui::{Color32, Rounding, Sense, Ui};
use egui_form::load_layout;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::new(MyApp)),
    )
}

struct MyApp;

fn test_ui(ui: &mut Ui) {
    ui.label("Test View");
    ui.label("So nice");
}

fn test2_ui(ui: &mut Ui) {
    if ui.button("Click me!").clicked() {
        ui.label("Huhhu Button Clicked");
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            load_layout!(
                <?xml version="1.0" encoding="utf-8"?>
                <Form>
                    <Strip direction="north">
                        <Panel size="relative" value="0.75">
                            <Strip direction="west">
                                <Panel size="exact" value="250.0">
                                </Panel>
                                <Panel size="remainder">
                                    <Rust>
                                        test_ui(ui);
                                    </Rust>
                                </Panel>
                            </Strip>
                        </Panel>
                        <Panel size="remainder">
                            <Rust>
                                test2_ui(ui);
                            </Rust>
                        </Panel>
                    </Strip>
                </Form>
            );
        });
    }
}
