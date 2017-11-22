use conrod::image;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;
pub mod item_history;
pub mod panel;
pub use custom_widget::image_panels::item_history::ItemHistory;
pub use custom_widget::image_panels::panel::ImagePanels;
pub trait Panelable {
    fn text(&self) -> Option<String>;
    fn display_pic(&self) -> Option<(image::Id, Option<([f64; 2], [f64; 2])>)>;
    fn list_image(&self) -> Vec<(image::Id, ([f64; 2], [f64; 2]))>;
    fn list_selected(&self) -> &mut HashSet<usize, RandomState>;
}
