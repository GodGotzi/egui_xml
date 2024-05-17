# egui_xml

`egui_xml` is a powerful Rust crate designed to enhance the `egui` library by providing a convenient macro to load and render user interface layouts from XML files. This crate streamlines the UI development process by allowing developers to define complex layouts in a structured and readable XML format, which is then dynamically interpreted and rendered by the `egui` framework.

## Key Features

- **XML-Based Layout Definition**: Define your UI layout in a clear and structured XML format, making it easier to visualize and manage complex interfaces.
- **Dynamic Value Insertion**: Seamlessly insert dynamic values and conditions into your XML layout using a straightforward syntax.
- **Uses StripBuilder from `egui_extras`**: Utilize custom panels and strips to organize your UI elements efficiently, supporting relative, exact, initial, and remainder sizing.

## Example Usage

Here's an example showcasing how to use the `egui_xml` crate to define a UI layout:

```rust
use eframe::egui;
use egui::{Rounding, Ui};
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

fn color_background(ui: &mut Ui, color: egui::Color32) {
    ui.painter()
        .rect_filled(ui.available_rect_before_wrap(), Rounding::same(5.0), color);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let dynamic_x = 5.0;
            let dynamic_if_separator = true;
            
            load_layout!(
                <?xml version="1.0" encoding="utf-8"?>
                <Form>
                    <Strip direction="north" gap="@dynamic_x" separator="@dynamic_if_separator">
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
                                    <Strip direction="north" gap="1.5">
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
                </Form>
            );
        });
    }
}
```

In this example, the load_layout! macro takes an XML string that defines the structure and style of the UI. Dynamic values and conditions can be injected directly into the XML, allowing for a flexible and dynamic UI creation process.
Getting Started

To get started with egui_xml, add the crate to your Cargo.toml:

```toml
[dependencies]
egui_xml = "0.1"
eframe = "0.13" # or the latest version
```

Then, you can start defining your UI layouts in XML and loading them using the load_layout! macro within your eframe application.

# Conclusion

egui_xml is an excellent tool for developers looking to leverage the power of XML to create and manage egui user interfaces efficiently. Its ability to dynamically handle values and conditions makes it a versatile choice for building rich, responsive UIs in Rust.


