//! The `Button` widget and related items.

use conrod::{Color, color, Colorable, FontSize, Borderable, Labelable, Positionable, Sizeable,
             UiCell, Widget, text, event, input, image, Theme};
use conrod::position::{self, Align, Rect, Scalar, Dimensions, Point};
use conrod::widget;
use conrod::widget::envelope_editor::EnvelopePoint;
use custom_widget::dragdrop_list::Draggable;
use std::time::{Duration, Instant};
/// The `Button` displays an `Image` on top.
#[derive(Copy, Clone)]
pub struct Image {
    /// The id of the `Image` to be used.
    pub image_id: image::Id,
    /// The image displayed when the button is held for 4 seconds.
    pub toggle_image_id: Option<image::Id>,
    /// The image overlay on the mouse while is held for more than 2 seconds
    pub spinner_image_id: Option<image::Id>,
}
/// A pressable button widget whose reaction is triggered upon release.
#[derive(WidgetCommon)]
pub struct Button<S> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// Whether the `Button` is a `Flat` color or an `Image`.
    pub show: S,
    /// Unique styling parameters for the Button.
    pub style: Style,
    /// Whether or not user input is enabled.
    enabled: bool,
}

/// Unique styling for the Button.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the Button's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the button
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// Dragable
    #[conrod(default="false")]
    pub draggable: Option<bool>,
}

/// The State of the Button widget that will be cached within the Ui.
pub struct ImageState {
    /// Track whether some sort of dragging is currently occurring.
    drag: Drag,
    ids: ImageIds,
    toggle_bool: bool,
}
/// Track whether some sort of dragging is currently occurring.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Drag {
    /// The drag is currently selecting a range of text.
    Selecting(usize, Point),
    None,
    Terminate,
}

widget_ids! {
    /// Identifiers for an image button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct ImageIds {
        image,
        label,
        rectangle,
        spinner
    }
}



impl<S> Button<S> {
    /// Create a button context to be built upon.
    fn new_internal(show: S) -> Self {
        Button {
            common: widget::CommonBuilder::default(),
            show: show,
            style: Style::default(),
            enabled: true,
        }
    }
}
impl Button<Image> {
    /// Begin building a button displaying the given `Image` on top.
    pub fn image(img: image::Id) -> Self {
        let image = Image {
            image_id: img,
            toggle_image_id: None,
            spinner_image_id: None,
        };
        Self::new_internal(image)
    }
    /// The image displayed when the button is held for 4 seconds.
    pub fn toggle_image(mut self, id: image::Id) -> Self {
        self.show.toggle_image_id = Some(id);
        self
    }
    /// The spinner image overlay displayed when the button is held for 2 seconds.
    pub fn spinner_image(mut self, id: image::Id) -> Self {
        self.show.spinner_image_id = Some(id);
        self
    }
}
impl Widget for Button<Image> {
    type State = ImageState;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        ImageState {
            drag: Drag::None,
            toggle_bool: false,
            ids: ImageIds::new(id_gen),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the Button.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let Button { show, .. } = self;
        let mut drag = state.drag;
        let mut toggle_bool = state.toggle_bool;
        update_drag(id, &mut drag, ui);
        bordered_rectangle(id,
                           state.ids.rectangle,
                           rect,
                           style.color(&ui.theme),
                           style,
                           ui);
        let draw_spinner_index = update_toggle_bool_spinner_index(&mut drag, &mut toggle_bool);
        state.update(|state| {
                         state.drag = drag;
                         state.toggle_bool = toggle_bool
                     });
        // Instantiate the image.
        let widget_image = if toggle_bool {
            show.toggle_image_id.unwrap()
        } else {
            show.image_id
        };
        let (x, y, w, h) = rect.x_y_w_h();
        let image = widget::Image::new(widget_image)
            .x_y(x, y)
            .top_right_with_margins_on(id, h * 0.2, w * 0.2)
            .w_h(w * 0.6, h * 0.6)
            .parent(id)
            .graphics_for(id);

        image.set(state.ids.image, ui);
        if let Some(spinner_index) = draw_spinner_index {
            draw_spinner_op(id,
                            state.ids.spinner,
                            show.spinner_image_id,
                            spinner_index,
                            ui);
        }
        ()
    }
    fn drag_area(&self, dim: Dimensions, style: &Style, theme: &Theme) -> Option<Rect> {
        if let Some(_) = style.draggable {
            Some(Rect::from_xy_dim([0.0, 0.0], dim))
        } else {
            None
        }
    }
}

fn update_drag(button_id: widget::Id, drag: &mut Drag, ui: &UiCell) {
    for widget_event in ui.widget_input(button_id).events() {
        match widget_event {
            event::Widget::Press(press) => {
                match press.button {
                    event::Button::Mouse(input::MouseButton::Left, point) => {
                        match drag {
                            &mut Drag::Selecting(a, point) => {
                                /*  if a >=60{
                                    *drag = Drag::Terminate;
                                } 
                                */
                            }
                            &mut Drag::None => {
                                *drag = Drag::Selecting(0, point);
                            }
                            &mut Drag::Terminate => {}
                        }
                    }
                    _ => {}
                }
            }
            event::Widget::Click(click) => {
                match (click, drag.clone()) {
                    (event::Click { button: input::MouseButton::Left, .. }, Drag::Terminate) => {
                        *drag = Drag::None;
                    }
                    _ => {}
                }
            }
            event::Widget::Release(release) => {
                if let event::Button::Mouse(input::MouseButton::Left, _) = release.button {
                    match drag {
                        &mut Drag::Selecting(_, _) => {
                            *drag = Drag::Terminate;
                        }
                        _ => {}
                    }
                }
            }
            event::Widget::Drag(drag_event) if drag_event.button == input::MouseButton::Left => {
                match drag {
                    &mut Drag::Selecting(_, ref mut point) => {
                        let dim = ui.wh_of(button_id).unwrap();
                        *point = drag_event.to;
                    }
                    _ => {}
                }
            }
            _ => if let Drag::None = *drag {},
        }
    }

}

fn bordered_rectangle(button_id: widget::Id,
                      rectangle_id: widget::Id,
                      rect: Rect,
                      color: Color,
                      style: &Style,
                      ui: &mut UiCell) {
    // BorderedRectangle widget.
    let dim = rect.dim();
    let border = style.border(&ui.theme);
    let border_color = style.border_color(&ui.theme);
    widget::BorderedRectangle::new(dim)
        .middle_of(button_id)
        .graphics_for(button_id)
        .color(color)
        .border(border)
        .border_color(border_color)
        .set(rectangle_id, ui);
}

fn draw_spinner_op(button_id: widget::Id,
                   spinner_id: widget::Id,
                   spinner_image: Option<image::Id>,
                   spinner_index: usize,
                   ui: &mut UiCell) {
    if let Some(spinner_image) = spinner_image {
        widget::Image::new(spinner_image)
            .source_rectangle(get_spriteinfo().src_rect(spinner_index as f64))
            .w_h(40.0, 40.0)
            .middle_of(button_id)
            .set(spinner_id, ui);
    }

}
fn update_toggle_bool_spinner_index(drag: &mut Drag, toggle_bool: &mut bool) -> Option<usize> {
    match drag {
        &mut Drag::Selecting(ref mut spinner_index, point) => {
            if *spinner_index >= 60 {
                if *toggle_bool {
                    *toggle_bool = false;
                } else {
                    *toggle_bool = true;
                }

                *spinner_index = 0;
                None
            } else {

                *spinner_index += 1;
                Some(spinner_index.clone())
            }
        }
        _ => None,
    }
}
impl<S> Colorable for Button<S> {
    builder_method!(color { style.color = Some(Color) });
}

impl<S> Borderable for Button<S> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
impl<S> Draggable for Button<S> {
    builder_methods!{
        draggable { style.draggable = Some(bool) }
    }
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: f64,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
pub fn get_spriteinfo() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 206.0),
        num_in_row: 12.0,
        w_h: (51.8333, 51.5),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
impl SpriteInfo {
    pub fn src_rect(&self, index: f64) -> Rect {
        let s = self;
        let (x, y) = (index % s.num_in_row as f64, (index / (s.num_in_row)).floor());
        let r = position::rect::Rect::from_corners([s.first.0 + x * s.w_h.0 + s.pad.0,
                                                    s.first.1 - y * s.w_h.1 - s.pad.2],
                                                   [s.first.0 + (x + 1.0) * s.w_h.0 - s.pad.1,
                                                    s.first.1 - (y + 1.0) * s.w_h.1 + s.pad.3]);
        r
    }
}