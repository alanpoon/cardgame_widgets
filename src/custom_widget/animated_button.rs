//! The `AnimatedButton` widget and related items.

use conrod::{Color, Colorable, FontSize, Borderable, Labelable, Positionable, Sizeable, UiCell,
             Widget, image, text, Range};
use conrod::position::{self, Align, Rect, Scalar};
use conrod::widget;


/// A pressable button widget whose reaction is triggered upon release.
#[derive(WidgetCommon)]
pub struct AnimatedButton<'a, S> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    maybe_label: Option<&'a str>,
    /// Whether the `AnimatedButton` is a `Flat` color or an `Image`.
    pub show: S,
    /// Unique styling parameters for the AnimatedButton.
    pub style: Style,
    /// Whether or not user input is enabled.
    enabled: bool,
}

/// Unique styling for the AnimatedButton.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the AnimatedButton's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the button
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// The color of the AnimatedButton's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The font size of the AnimatedButton's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<FontSize>,
    /// The ID of the font used to display the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<text::font::Id>>,
    /// The label's typographic alignment over the *x* axis.
    #[conrod(default = "text::Justify::Center")]
    pub label_justify: Option<text::Justify>,
    /// The position of the title bar's `Label` widget over the *x* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub label_x: Option<position::Relative>,
    /// The position of the title bar's `Label` widget over the *y* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub label_y: Option<position::Relative>,
}

widget_ids! {
    /// Identifiers for a "flat" button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct FlatIds {
        rectangle,
        label,
    }
}

widget_ids! {
    /// Identifiers for an image button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct ImageIds {
        image,
        label,
    }
}

/// The `AnimatedButton` simply displays a flat color.
#[derive(Copy, Clone)]
pub struct Flat;

/// The `AnimatedButton` displays an `Image` on top.
#[derive(Copy, Clone)]
pub struct Image {
    /// The id of the `Image` to be used.
    pub image_id: image::Id,
    pub normal_rect: Rect,
    /// The image displayed when the mouse hovers over the button.
    pub hover_rect: Rect,
    /// The image displayed when the mouse has captured and is pressing the button.
    pub press_rect: Rect,
}

#[derive(Copy, Clone)]
enum Interaction {
    Idle,
    Hover,
    Press,
}

/// The `Event` type yielded by the `AnimatedButton` widget.
///
/// Represents the number of times that the `AnimatedButton` has been clicked with the left mouse button
/// since the last update.
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct TimesClicked(pub u16);


impl TimesClicked {
    /// `true` if the `AnimatedButton` was clicked one or more times.
    pub fn was_clicked(self) -> bool {
        self.0 > 0
    }
}

impl Iterator for TimesClicked {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 0 {
            self.0 -= 1;
            Some(())
        } else {
            None
        }
    }
}


impl<'a> AnimatedButton<'a, Image> {
    /// Begin building a button displaying the given `Image` on top.
    pub fn image(image_id: image::Id) -> Self {
        let image = Image {
            image_id: image_id,
            normal_rect: Rect {
                x: Range::new(0.0, 200.0),
                y: Range::new(0.0, 200.0),
            },
            hover_rect: Rect {
                x: Range::new(0.0, 200.0),
                y: Range::new(0.0, 200.0),
            },
            press_rect: Rect {
                x: Range::new(0.0, 200.0),
                y: Range::new(0.0, 200.0),
            },
        };
        Self::new_internal(image)
    }
    pub fn normal_rect(mut self, src_rect: Rect) -> Self {
        self.show.normal_rect = src_rect;
        self
    }
    /// The image displayed while the mouse hovers over the `AnimatedButton`.
    pub fn hover_rect(mut self, src_rect: Rect) -> Self {
        self.show.hover_rect = src_rect;
        self
    }

    /// The image displayed while the `AnimatedButton` is pressed.
    pub fn press_rect(mut self, src_rect: Rect) -> Self {
        self.show.press_rect = src_rect;
        self
    }
}

impl<'a> AnimatedButton<'a, Flat> {
    /// Begin building a flat-colored `AnimatedButton` widget.
    pub fn new() -> Self {
        Self::new_internal(Flat)
    }

    /// Override the default button style
    pub fn with_style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }
}


impl<'a, S> AnimatedButton<'a, S> {
    /// Create a button context to be built upon.
    fn new_internal(show: S) -> Self {
        AnimatedButton {
            common: widget::CommonBuilder::default(),
            show: show,
            maybe_label: None,
            style: Style::default(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// Align the label to the left of the `AnimatedButton`'s surface.
    pub fn left_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Left);
        self
    }

    /// Align the label to the mid-left of the `AnimatedButton`'s surface.
    ///
    /// This is the default label alignment.
    pub fn center_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Center);
        self
    }

    /// Align the label to the mid-left of the `AnimatedButton`'s surface.
    pub fn right_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Right);
        self
    }

    /// Specify the label's position relatively to `AnimatedButton` along the *x* axis.
    pub fn label_x(mut self, x: position::Relative) -> Self {
        self.style.label_x = Some(x);
        self
    }

    /// Specify the label's position relatively to `AnimatedButton` along the *y* axis.
    pub fn label_y(mut self, y: position::Relative) -> Self {
        self.style.label_y = Some(y);
        self
    }

    builder_methods!{
        pub enabled { enabled = bool }
    }
}


impl<'a> Widget for AnimatedButton<'a, Flat> {
    type State = FlatIds;
    type Style = Style;
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        FlatIds::new(id_gen)
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the AnimatedButton.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let AnimatedButton { maybe_label, .. } = self;

        let (interaction, times_triggered) = interaction_and_times_triggered(id, ui);
        let color = color_from_interaction(style.color(&ui.theme), interaction);

        bordered_rectangle(id, state.rectangle, rect, color, style, ui);

        // Label widget.
        if let Some(l) = maybe_label {
            label(id, state.label, l, style, ui);
        }

        TimesClicked(times_triggered)
    }
}

impl<'a> Widget for AnimatedButton<'a, Image> {
    type State = ImageIds;
    type Style = Style;
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        ImageIds::new(id_gen)
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the AnimatedButton.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let AnimatedButton { show, maybe_label, .. } = self;

        let (interaction, times_triggered) = interaction_and_times_triggered(id, ui);

        // Instantiate the image.
        let Image { image_id, normal_rect, press_rect, hover_rect } = show;

        // Determine the correct image to display.
        let s_rect = match interaction {
            Interaction::Idle => normal_rect,
            Interaction::Hover => hover_rect,
            Interaction::Press => press_rect,
        };

        let (x, y, w, h) = rect.x_y_w_h();
        let image = widget::Image::new(image_id)
            .source_rectangle(s_rect)
            .x_y(x, y)
            .w_h(w, h)
            .parent(id)
            .graphics_for(id);
        image.set(state.image, ui);

        if let Some(s) = maybe_label {
            label(id, state.label, s, style, ui);
        }

        TimesClicked(times_triggered)
    }
}


fn color_from_interaction(color: Color, interaction: Interaction) -> Color {
    match interaction {
        Interaction::Idle => color,
        Interaction::Hover => color.highlighted(),
        Interaction::Press => color.clicked(),
    }
}

fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {
    let input = ui.widget_input(button_id);
    let interaction = input.mouse().map_or(Interaction::Idle,
                                           |mouse| if mouse.buttons.left().is_down() {
                                               Interaction::Press
                                           } else {
                                               Interaction::Hover
                                           });
    let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
    (interaction, times_triggered)
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

fn label(button_id: widget::Id,
         label_id: widget::Id,
         label: &str,
         style: &Style,
         ui: &mut UiCell) {
    let color = style.label_color(&ui.theme);
    let font_size = style.label_font_size(&ui.theme);
    let x = style.label_x(&ui.theme);
    let y = style.label_y(&ui.theme);
    let justify = style.label_justify(&ui.theme);
    let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
    widget::Text::new(label)
        .and_then(font_id, widget::Text::font_id)
        .x_position_relative_to(button_id, x)
        .y_position_relative_to(button_id, y)
        .justify(justify)
        .parent(button_id)
        .graphics_for(button_id)
        .color(color)
        .font_size(font_size)
        .set(label_id, ui);
}


impl<'a, S> Colorable for AnimatedButton<'a, S> {
    builder_method!(color { style.color = Some(Color) });
}

impl<'a, S> Borderable for AnimatedButton<'a, S> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}

impl<'a, S> Labelable<'a> for AnimatedButton<'a, S> {
    builder_methods!{
        label { maybe_label = Some(&'a str) }
        label_color { style.label_color = Some(Color) }
        label_font_size { style.label_font_size = Some(FontSize) }
    }
}
