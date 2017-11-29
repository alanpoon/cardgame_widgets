use conrod::image;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;
pub mod item_history;
pub mod panel;
pub mod list_select;
pub use custom_widget::image_panels::item_history::ItemHistory;
pub use custom_widget::image_panels::panel::ImagePanels;
pub type ImageRectType = (image::Id, Option<([f64; 2], [f64; 2])>,usize); //usize is the index
pub trait Panelable {
    fn text(&self) -> Option<String>;
    fn display_pic(&self) -> Option<ImageRectType>;
    fn list_image(&self) -> Vec<ImageRectType>;
    fn list_selected<'a>(&'a self) -> &'a HashSet<usize, RandomState>;
    fn list_selected_mut<'a>(&'a mut self) -> &'a mut HashSet<usize, RandomState>;
}
