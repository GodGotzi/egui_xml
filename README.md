# egui_xml
[<img alt="github" src="https://img.shields.io/badge/github-godgotzi/egui_xml-8da0cb?logo=github" height="20">](https://github.com/godgotzi/egui_xml)
[![Latest Version](https://img.shields.io/crates/v/egui_xml.svg)](https://crates.io/crates/egui_xml)
[![Documentation](https://docs.rs/egui_xml/badge.svg)](https://docs.rs/egui_xml)
[![License](https://img.shields.io/crates/l/egui_xml.svg)](https://github.com/godgotzi/egui_xml#license)

---

`egui_xml` is a powerful Rust crate designed to enhance the `egui` library by providing a convenient macro to load user interface layouts from XML files. This crate streamlines the UI development process by allowing developers to define complex layouts in a structured and readable XML format.

## Key Features

- **XML-Based Layout Definition**: Define your UI layout in a clear and structured XML format, making it easier to visualize and manage complex interfaces.
- **Dynamic Value Insertion**: Seamlessly insert dynamic values and conditions into your XML layout using a straightforward syntax.
- **Uses StripBuilder from `egui_extras`**: Utilize custom panels and strips to organize your UI elements efficiently, supporting relative, exact, initial, and remainder sizing.
- **load_layout_file macro**: Load Layout from a file

## Example Usage

Here's an example showcasing how to use the `egui_xml` crate to define a UI layout:

```rust
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
        Box::new(|_cc| Box::<MyApp>::new(MyApp)),
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
            load_layout!(
                <Strip direction="west">
                    <Panel size="relative" value="0.3">
                        color_background(ui, egui::Color32::from_rgb(0, 0, 255));
                    </Panel>
                    <Panel size="remainder">
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
```

### Expanded

```rust
use eframe::egui;
use egui::{Rounding, Ui};
use egui_xml::load_layout;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::<MyApp>::new(MyApp)))
}

struct MyApp;

fn color_background(ui: &mut Ui, color: egui::Color32) {
    ui.painter()
        .rect_filled(ui.available_rect_before_wrap(), Rounding::same(5.0), color);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(
                ctx,
                |ui| {
                    let mut macro_strip_builder = egui_extras::StripBuilder::new(ui);
                    macro_strip_builder = macro_strip_builder
                        .size(egui_extras::Size::relative(0.3));
                    macro_strip_builder = macro_strip_builder
                        .size(egui_extras::Size::remainder());
                    let macro_strip_response = macro_strip_builder
                        .horizontal(|mut strip| {
                            strip
                                .cell(|ui| {
                                    color_background(ui, egui::Color32::from_rgb(0, 0, 255));
                                });
                            strip
                                .cell(|ui| {
                                    let mut macro_strip_builder = egui_extras::StripBuilder::new(
                                        ui,
                                    );
                                    macro_strip_builder = macro_strip_builder
                                        .size(egui_extras::Size::relative(0.3));
                                    macro_strip_builder = macro_strip_builder
                                        .size(egui_extras::Size::remainder());
                                    let macro_strip_response = macro_strip_builder
                                        .vertical(|mut strip| {
                                            strip
                                                .cell(|ui| {
                                                    color_background(ui, egui::Color32::from_rgb(0, 255, 255));
                                                });
                                            strip
                                                .cell(|ui| {
                                                    color_background(ui, egui::Color32::from_rgb(255, 0, 255));
                                                });
                                        });
                                });
                        });
                },
            );
    }
}
```

In this example, the load_layout! macro takes an XML string that creates the StripBuilder code for the UI. Dynamic values and conditions can be injected directly into the XML, allowing for a flexible and dynamic UI creation process.
Getting Started

To get started with egui_xml, add the crate to your Cargo.toml:

```toml
[dependencies]
egui_xml = "0.1"
egui = "0.27"
```

Then, you can start defining your UI layouts in XML and loading them using the load_layout! macro within your eframe application.

# Conclusion

egui_xml is an excellent tool for developers looking to leverage the power of XML to create and manage egui user interfaces efficiently. Its ability to dynamically handle values and conditions makes it a versatile choice for building rich, responsive UIs in Rust.


