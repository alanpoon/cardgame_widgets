use conrod::{Scalar, FontSize};
use std::cmp::min;
const LABEL_PADDING: f64 = 4.0;
/// Return the dimensions of a value glyph slot.
pub fn value_glyph_slot_width(size: FontSize) -> f64 {
    (size as f64 * 0.75).floor() as f64
}
/// Return the dimensions of value string glyphs.
pub fn calc_width(font_size: FontSize, val_string: &String) -> f64 {
    let slot_w = value_glyph_slot_width(font_size);
    let val_string_w = slot_w * val_string.len() as f64;
    val_string_w
}
pub fn get_font_size_wh(w: Scalar, h: Scalar, val_string: &str) -> FontSize {
    min((w / (val_string.len() as f64) / 0.75).floor() as u32,
        (h - 2.0 * LABEL_PADDING).floor() as u32)
}
pub fn get_font_size_hn(h: Scalar, numberline: Scalar, val_string: &str) -> FontSize {
    (h / numberline - 2.0 * LABEL_PADDING).floor() as u32
}
/// Calculate the default height for the **TitleBar**'s rect.
pub fn calc_height(font_size: FontSize) -> Scalar {
    font_size as Scalar + LABEL_PADDING * 2.0
}
