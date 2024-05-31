//! `load_layout!` macro for loading layout from XML for egui
//!
//! The `load_layout!` macro allows for loading layout configurations from XML for use with the egui GUI framework. It takes an XML representation of the layout structure and generates Rust code to construct the layout within an egui UI.
//!
//! # Example
//!
//! ```rust
//! # use egui_macros::load_layout;
//! let layout_code = load_layout!(
//!     <Strip direction="west">
//!         <Panel size="relative" value="0.3">
//!             color_background(ui, egui::Color32::from_rgb(0, 0, 255));
//!         </Panel>
//!         <Panel size="remainder">
//!             <Strip direction="north">
//!                 <Panel size="relative" value="0.3">
//!                     color_background(ui, egui::Color32::from_rgb(0, 255, 255));
//!                 </Panel>
//!                 <Panel size="remainder">
//!                     color_background(ui, egui::Color32::from_rgb(255, 0, 255));
//!                 </Panel>
//!             </Strip>
//!         </Panel>
//!     </Strip>
//! );
//! ```

pub use egui_xml_macros::load_layout;
pub use egui_xml_macros::load_layout_file;
