use eframe::egui;
use macros::load_layout_file;

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

fn color_background(ui: &mut egui::Ui, color: egui::Color32) {
    ui.painter().rect_filled(
        ui.available_rect_before_wrap(),
        egui::Rounding::same(5.0),
        color,
    );
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let vertical_gap_symetric = 1.5;

            load_layout_file!("tests/strip.xml");
        });
    }
}
