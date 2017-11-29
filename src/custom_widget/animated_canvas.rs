//! The `Canvas` widget and related items.

use conrod::{Color, Colorable, FontSize, Borderable, Labelable, Positionable, Sizeable, Theme, Ui,
             UiCell, Widget, image};
use conrod::position::{self, Dimensions, Padding, Place, Position, Range, Rect, Scalar};
use conrod::position::Direction::{Forwards, Backwards};
use conrod::text;
use conrod::widget;
use std::marker::Send;
use std::fmt::Debug;
/// **Canvas** is designed to be a "container"-like "parent" widget that simplifies placement of
/// "children" widgets.
///
/// Widgets can be placed on a **Canvas** in a variety of ways using methods from the
/// [**Positionable**](../position/trait.Positionable) trait.
///
/// **Canvas** provides methods for padding the kid widget area which can make using the
/// **Place**-related **Positionable** methods a little easier.
///
/// A **Canvas** can also be divided into a sequence of smaller **Canvas**ses using the `.flow_*`
/// methods. This creates a kind of **Canvas** tree, where each "split" can be sized using the
/// `.length` or `.length_weight` methods.
///
/// See the `canvas.rs` example for a demonstration of the **Canvas** type.
#[derive(Copy, Clone, Debug, WidgetCommon)]
pub struct Canvas<'a, T>
    where T: Clone + PartialEq + 'a + Send + 'static + Debug
{
    /// Data necessary and common for all widget builder types.
    #[conrod(common_builder)]
    pub common: widget::CommonBuilder,
    /// The builder data related to the style of the Canvas.
    pub style: Style,
    /// The label for the **Canvas**' **TitleBar** if there is one.
    pub maybe_title_bar_label: Option<&'a str>,
    /// A list of child **Canvas**ses as splits of this **Canvas** flowing in the given direction.
    pub maybe_splits: Option<FlowOfSplits<'a, T>>,
    /// Watch State, used to compare with the cached state
    pub watch_state: Option<T>,
    /// close icon image Id
    pub close_icon: Option<image::Id>,
    /// close icon image source rectange
    pub close_icon_src_rect: Option<Rect>,
}

/// **Canvas** state to be cached.
pub struct State<T> {
    ids: Ids,
    state: Option<T>,
    frame: f64,
    closing: bool,
}

widget_ids! {
    struct Ids {
        rectangle,
        title_bar,
        close_icon
    }
}

/// Unique styling for the Canvas.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// The color of the Canvas' rectangle surface.
    #[conrod(default = "theme.background_color")]
    pub color: Option<Color>,
    /// The width of the border surrounding the Canvas' rectangle.
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the Canvas' border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// If this Canvas is a split of some parent Canvas, this is the length of the split.
    #[conrod(default = "Length::Weight(1.0)")]
    pub length: Option<Length>,

    /// Padding for the left edge of the Canvas' kid area.
    #[conrod(default = "theme.padding.x.start")]
    pub pad_left: Option<Scalar>,
    /// Padding for the right edge of the Canvas' kid area.
    #[conrod(default = "theme.padding.x.end")]
    pub pad_right: Option<Scalar>,
    /// Padding for the bottom edge of the Canvas' kid area.
    #[conrod(default = "theme.padding.y.start")]
    pub pad_bottom: Option<Scalar>,
    /// Padding for the top edge of the Canvas' kid area.
    #[conrod(default = "theme.padding.y.end")]
    pub pad_top: Option<Scalar>,

    /// The color of the title bar. Defaults to the color of the Canvas.
    #[conrod(default = "None")]
    pub title_bar_color: Option<Option<Color>>,
    /// The color of the title bar's text.
    #[conrod(default = "theme.label_color")]
    pub title_bar_text_color: Option<Color>,
    /// The font size for the title bar's text.
    #[conrod(default = "theme.font_size_medium")]
    pub title_bar_font_size: Option<FontSize>,
    /// The way in which the title bar's text should wrap.
    #[conrod(default = "Some(widget::text::Wrap::Whitespace)")]
    pub title_bar_maybe_wrap: Option<Option<widget::text::Wrap>>,
    /// The distance between lines for multi-line title bar text.
    #[conrod(default = "1.0")]
    pub title_bar_line_spacing: Option<Scalar>,
    /// The label's typographic alignment over the *x* axis.
    #[conrod(default = "text::Justify::Center")]
    pub title_bar_justify: Option<text::Justify>,
    /// The number of frames to finish transition
    #[conrod(default = "60")]
    pub frame_rate: Option<usize>,
    /// close icon dimension
    #[conrod(default="[80.0,80.0]")]
    pub close_icon_dim: Option<[f64; 2]>,
    /// close icon position top right of corner
    #[conrod(default="[90.0,90.0]")]
    pub close_icon_pos: Option<[f64; 2]>,
}

/// A series of **Canvas** splits along with their unique identifiers.
pub type ListOfSplits<'a, T> = &'a [(widget::Id, Canvas<'a, T>)];

/// A series of **Canvas** splits flowing in the specified direction.
pub type FlowOfSplits<'a, T> = (Direction, ListOfSplits<'a, T>);

/// The length of a `Split` given as a weight.
///
/// The length is determined by determining what percentage each `Split`'s weight contributes to
/// the total weight of all `Split`s in a flow list.
pub type Weight = Scalar;

/// Used to describe the desired length for a `Split`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    /// The length as an absolute scalar.
    Absolute(Scalar),
    /// The length as a weight of the non-absolute length of the parent **Canvas**.
    Weight(Weight),
}

/// The direction in which a sequence of canvas splits will be laid out.
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    /// Lay splits along the *x* axis.
    X(position::Direction),
    /// Lay splits along the *y* axis.
    Y(position::Direction),
}


impl<'a, T> Canvas<'a, T>
    where T: Clone + PartialEq + 'a + Send + 'static + Debug
{
    /// Construct a new Canvas builder.
    pub fn new() -> Self {
        Canvas {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            maybe_title_bar_label: None,
            maybe_splits: None,
            watch_state: None,
            close_icon: None,
            close_icon_src_rect: None,
        }
    }

    builder_methods!{
        pub title_bar { maybe_title_bar_label = Some(&'a str) }
        pub pad_left { style.pad_left = Some(Scalar) }
        pub pad_right { style.pad_right = Some(Scalar) }
        pub pad_bottom { style.pad_bottom = Some(Scalar) }
        pub pad_top { style.pad_top = Some(Scalar) }
        pub with_style { style = Style }
        pub frame_rate {style.frame_rate = Some(usize)}
        pub close_icon_dim {style.close_icon_dim = Some([f64;2])}
        pub close_icon_pos {style.close_icon_pos=Some([f64;2])}
    }

    /// Set the length of the Split as an absolute scalar.
    pub fn length(mut self, length: Scalar) -> Self {
        self.style.length = Some(Length::Absolute(length));
        self
    }

    /// Set the length of the Split as a weight.
    ///
    /// The default length weight for each widget is `1.0`.
    pub fn length_weight(mut self, weight: Weight) -> Self {
        self.style.length = Some(Length::Weight(weight));
        self
    }

    /// Set the child Canvas Splits of the current Canvas flowing in a given direction.
    fn flow(mut self, direction: Direction, splits: ListOfSplits<'a, T>) -> Self {
        self.maybe_splits = Some((direction, splits));
        self
    }

    /// Set the child Canvasses flowing to the right.
    pub fn flow_right(self, splits: ListOfSplits<'a, T>) -> Self {
        self.flow(Direction::X(Forwards), splits)
    }

    /// Set the child Canvasses flowing to the left.
    pub fn flow_left(self, splits: ListOfSplits<'a, T>) -> Self {
        self.flow(Direction::X(Backwards), splits)
    }

    /// Set the child Canvasses flowing upwards.
    pub fn flow_up(self, splits: ListOfSplits<'a, T>) -> Self {
        self.flow(Direction::Y(Forwards), splits)
    }

    /// Set the child Canvasses flowing downwards.
    pub fn flow_down(self, splits: ListOfSplits<'a, T>) -> Self {
        self.flow(Direction::Y(Backwards), splits)
    }

    /// Set the padding for all edges of the area where child widgets will be placed.
    #[inline]
    pub fn pad(self, pad: Scalar) -> Self {
        self.pad_left(pad)
            .pad_right(pad)
            .pad_bottom(pad)
            .pad_top(pad)
    }

    /// Set the padding of the area where child widgets will be placed.
    #[inline]
    pub fn padding(self, pad: Padding) -> Self {
        self.pad_left(pad.x.start)
            .pad_right(pad.x.end)
            .pad_bottom(pad.y.start)
            .pad_top(pad.y.end)
    }

    /// Set the color of the `Canvas`' `TitleBar` if it is visible.
    pub fn title_bar_color(mut self, color: Color) -> Self {
        self.style.title_bar_color = Some(Some(color));
        self
    }
    /// Set the watch state to match with the Widget's state
    pub fn watch_state(mut self, state: T) -> Self {
        self.watch_state = Some(state);
        self
    }
    /// Set an image Id for the close icon
    pub fn close_icon(mut self, _image: image::Id) -> Self {
        self.close_icon = Some(_image);
        self
    }
    /// Set a Rect for the close icon's image source rectangle
    pub fn close_icon_src_rect(mut self, _rect: Rect) -> Self {
        self.close_icon_src_rect = Some(_rect);
        self
    }
}


impl<'a, T> Widget for Canvas<'a, T>
    where T: Clone + PartialEq + 'a + Send + 'static + Debug
{
    type State = State<T>;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            state: None,
            frame: 0.0,
            closing: false,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn default_x_position(&self, _ui: &Ui) -> Position {
        Position::Relative(position::Relative::Place(Place::Middle), None)
    }

    fn default_y_position(&self, _ui: &Ui) -> Position {
        Position::Relative(position::Relative::Place(Place::Middle), None)
    }

    /// The title bar area at which the Canvas can be clicked and dragged.
    ///
    /// Note: the position of the returned **Rect** should be relative to the center of the widget.
    fn drag_area(&self, dim: Dimensions, style: &Style, theme: &Theme) -> Option<Rect> {
        self.maybe_title_bar_label.map(|_| {
            let font_size = style.title_bar_font_size(theme);
            let (h, rel_y) = title_bar_h_rel_y(dim[1], font_size);
            let rel_xy = [0.0, rel_y];
            let dim = [dim[0], h];
            Rect::from_xy_dim(rel_xy, dim)
        })
    }

    /// The area of the widget below the title bar, upon which child widgets will be placed.
    fn kid_area(&self, args: widget::KidAreaArgs<Self>) -> widget::KidArea {
        let widget::KidAreaArgs { rect, style, theme, .. } = args;
        if self.maybe_title_bar_label.is_some() {
            let font_size = style.title_bar_font_size(theme);
            let title_bar = title_bar(rect, font_size);
            widget::KidArea {
                rect: rect.pad_top(title_bar.h()),
                pad: style.padding(theme),
            }
        } else {
            widget::KidArea {
                rect: rect,
                pad: style.padding(theme),
            }
        }
    }

    /// Update the state of the Canvas.
    fn update(self, args: widget::UpdateArgs<Self>) {
        let widget::UpdateArgs { id, state, rect, mut ui, .. } = args;
        let Canvas { style, maybe_title_bar_label, maybe_splits, .. } = self;

        let watch_c = self.watch_state.clone();

        if let &None = &state.state {
            state.update(|state| state.state = watch_c.clone());
        }
        let state_c = state.state.clone();
        if let (&Some(ref _state), &Some(ref _watch)) = (&state_c, &self.watch_state) {
            if _state != _watch {
                state.update(|state| {
                                 state.frame = 0.0;
                                 state.state = Some(_watch.clone());
                             });
            } else {
                state.update(|state| state.frame += 1.0);
            }
        }
        // BorderedRectangle widget as the rectangle backdrop.
        let dim = rect.dim();
        let color = style.color(ui.theme());
        let border = style.border(ui.theme());
        let border_color = style.border_color(ui.theme());
        let frame_rate = style.frame_rate(ui.theme()) as f64;
        let _step = if state.frame > frame_rate {
            1.0
        } else {
            if state.closing {
                1.0 - state.frame / frame_rate
            } else {
                state.frame / frame_rate
            }
        };
        widget::BorderedRectangle::new(dim)
            .color(color)
            .border(border)
            .border_color(border_color)
            .middle_of(id)
            .graphics_for(id)
            .place_on_kid_area(false)
            .set(state.ids.rectangle, &mut ui);

        // TitleBar widget if we were given some label.
        if let Some(label) = maybe_title_bar_label {
            let color = style.title_bar_color(&ui.theme).unwrap_or(color);
            let font_size = style.title_bar_font_size(&ui.theme);
            let label_color = style.title_bar_text_color(&ui.theme);
            let justify = style.title_bar_justify(&ui.theme);
            let line_spacing = style.title_bar_line_spacing(&ui.theme);
            let maybe_wrap = style.title_bar_maybe_wrap(&ui.theme);
            widget::TitleBar::new(label, state.ids.rectangle)
                .and_mut(|title_bar| {
                             title_bar.style.maybe_wrap = Some(maybe_wrap);
                             title_bar.style.justify = Some(justify);
                         })
                .color(color)
                .border(border)
                .border_color(border_color)
                .label_font_size(font_size)
                .label_color(label_color)
                .line_spacing(line_spacing)
                .graphics_for(id)
                .place_on_kid_area(false)
                .set(state.ids.title_bar, &mut ui);
        }
        let mut first_split_id = None;
        // If we were given some child canvas splits, we should instantiate them.
        if let Some((direction, splits)) = maybe_splits {

            let (total_abs, total_weight) =
                splits.iter().fold((0.0, 0.0), |(abs, weight), &(_, ref split)| {
                    match split.clone().style.length(ui.theme()) {
                        Length::Absolute(a) => (abs + a, weight),
                        Length::Weight(w) => (abs, weight + w),
                    }
                });

            // No need to calculate kid_area again, we'll just get it from the graph.
            let kid_area = ui.kid_area_of(id).expect("No KidArea found");
            let kid_area_range = match direction {
                Direction::X(_) => kid_area.x,
                Direction::Y(_) => kid_area.y,
            };

            let total_length = kid_area_range.len();
            let non_abs_length = (total_length - total_abs).max(0.0);
            let weight_normaliser = 1.0 / total_weight;

            let length = |split: &Self, ui: &UiCell| -> Scalar {
                match split.style.length(ui.theme()) {
                    Length::Absolute(length) => length,
                    Length::Weight(weight) => weight * weight_normaliser * non_abs_length,
                }
            };

            let set_split = |split_id: widget::Id, split: Canvas<'a, T>, ui: &mut UiCell| {
                split.parent(id).set(split_id, ui);
            };

            // Instantiate each of the splits, matching on the direction first for efficiency.
            match direction {

                Direction::X(direction) => {
                    match direction {
                        Forwards => {
                            for (i, &(ref split_id, ref split)) in splits.iter().enumerate() {
                                let w = length(&split, &ui);
                                let split = match i {
                                        0 => {
                                            first_split_id = Some(split_id.clone());
                                            split.clone().h(kid_area.h()).mid_left_of(id)
                                        }
                                        _ => split.clone().right(0.0),
                                    }
                                    .w(w * _step);
                                set_split(split_id.clone(), split, &mut ui);
                            }
                        }
                        Backwards => {
                            for (i, &(ref split_id, ref split)) in splits.iter().enumerate() {
                                let w = length(&split, &ui);
                                let split = match i {
                                        0 => {
                                            first_split_id = Some(split_id.clone());
                                            split.clone().h(kid_area.h()).mid_right_of(id)
                                        }
                                        _ => split.clone().left(0.0),
                                    }
                                    .w(w * _step);
                                set_split(split_id.clone(), split, &mut ui);
                            }
                        }
                    }
                }

                Direction::Y(direction) => {
                    match direction {
                        Forwards => {
                            for (i, &(ref split_id, ref split)) in splits.iter().enumerate() {
                                let h = length(&split, &ui);
                                let split = match i {
                                        0 => {
                                            first_split_id = Some(split_id.clone());
                                            split.clone().w(kid_area.w()).mid_bottom_of(id)
                                        }
                                        _ => split.clone().up(0.0),
                                    }
                                    .h(h * _step);
                                set_split(split_id.clone(), split, &mut ui);
                            }
                        }
                        Backwards => {
                            for (i, &(ref split_id, ref split)) in splits.iter().enumerate() {
                                let h = length(&split, &ui);
                                let split = match i {
                                        0 => {
                                            first_split_id = Some(split_id.clone());
                                            split.clone().w(kid_area.w()).mid_top_of(id)
                                        }
                                        _ => split.clone().down(0.0),
                                    }
                                    .h(h * _step);
                                set_split(split_id.clone(), split, &mut ui);
                            }
                        }
                    }
                }
            }

        }
        if let Some(_close_image) = self.close_icon {
            let close_icon_dim = self.style.close_icon_dim(ui.theme());
            let close_icon_pos = self.style.close_icon_pos(ui.theme());
            if widget::Button::image(_close_image)
                   .w_h(close_icon_dim[0] * _step, close_icon_dim[1] * _step)
                   .top_right_with_margins_on(first_split_id.unwrap_or(id),
                                              close_icon_pos[1] * _step,
                                              close_icon_pos[0] * _step)
                   .border(0.0)
                   .set(state.ids.close_icon, ui)
                   .was_clicked() {
                state.update(|state| {
                                 state.frame = 0.0;
                                 state.closing = true;
                             });
            }
        }
    }
}


/// The height and relative y coordinate of a Canvas' title bar given some canvas height and font
/// size for the title bar.
fn title_bar_h_rel_y(canvas_h: Scalar, font_size: FontSize) -> (Scalar, Scalar) {
    let h = widget::title_bar::calc_height(font_size);
    let rel_y = canvas_h / 2.0 - h / 2.0;
    (h, rel_y)
}

/// The Rect for the Canvas' title bar.
fn title_bar(canvas: Rect, font_size: FontSize) -> Rect {
    let (c_w, c_h) = canvas.w_h();
    let (h, rel_y) = title_bar_h_rel_y(c_h, font_size);
    let xy = [0.0, rel_y];
    let dim = [c_w, h];
    Rect::from_xy_dim(xy, dim)
}

impl Style {
    /// Get the Padding for the Canvas' kid area.
    pub fn padding(&self, theme: &Theme) -> position::Padding {
        position::Padding {
            x: Range::new(self.pad_left(theme), self.pad_right(theme)),
            y: Range::new(self.pad_bottom(theme), self.pad_top(theme)),
        }
    }
}

impl<'a, T> Colorable for Canvas<'a, T>
    where T: Clone + PartialEq + 'a + Send + 'static + Debug
{
    builder_method!(color { style.color = Some(Color) });
}

impl<'a, T> Borderable for Canvas<'a, T>
    where T: Clone + PartialEq + 'a + Send + 'static + Debug
{
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}

impl<'a, T> Labelable<'a> for Canvas<'a, T>
    where T: Clone + PartialEq + 'a + Send + 'static + Debug
{
    fn label(self, text: &'a str) -> Self {
        self.title_bar(text)
    }
    builder_methods!{
        label_color { style.title_bar_text_color = Some(Color) }
        label_font_size { style.title_bar_font_size = Some(FontSize) }
    }
}
