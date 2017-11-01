//! The `Button` widget and related items.

use conrod::{Color, color, Colorable, FontSize, Borderable, Labelable, Positionable, Sizeable,
             UiCell, Widget, text, event, input, image, Theme};
use conrod::position::{self, Align, Rect, Scalar, Dimensions};
use conrod::widget;
use conrod::widget::envelope_editor::EnvelopePoint;
use std::time::{Duration, Instant};
/// The `Button` displays an `Image` on top.
#[derive(Copy, Clone)]
pub struct Image {
    /// The id of the `Image` to be used.
    pub image_id: image::Id,
    /// The image displayed when the mouse hovers over the button.
    pub toggle_image_id: Option<image::Id>,
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
}

/// The State of the Button widget that will be cached within the Ui.
pub struct ImageState {
    /// Track whether some sort of dragging is currently occurring.
    drag: Drag,
    ids: ImageIds,
}
/// Track whether some sort of dragging is currently occurring.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Drag {
    /// The drag is currently selecting a range of text.
    Selecting(Instant),
    None,
    Terminate,
}

widget_ids! {
    /// Identifiers for an image button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct ImageIds {
        image,
        label,
        rectangle
    }
}

#[derive(Copy, Clone,Debug)]
pub enum Interaction {
    Idle,
    Hover,
    Press,
    Hold,
}

/// The `Event` type yielded by the `Button` widget.
///
/// Represents the number of times that the `Button` has been clicked with the left mouse button
/// since the last update.
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct TimesClicked(pub Interaction);


impl TimesClicked {
    /// `true` if the `Button` was clicked one or more times.
    pub fn was_clicked(self) -> bool {
        if let Interaction::Press = self.0 {
            true
        } else {
            false
        }
    }
    pub fn was_hold(self) -> bool {
        if let Interaction::Hold = self.0 {
            true
        } else {
            false
        }
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
        };
        Self::new_internal(image)
    }
    /// The image displayed while the mouse hovers over the `Button`.
    pub fn toggle_image(mut self, id: image::Id) -> Self {
        self.show.toggle_image_id = Some(id);
        self
    }
}
impl Widget for Button<Image> {
    type State = ImageState;
    type Style = Style;
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        ImageState {
            drag: Drag::None,
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
        let interaction = interaction_and_times_triggered(id, &mut drag, ui);
        let color = color_from_interaction(style.color(&ui.theme), interaction, &mut drag);
        bordered_rectangle(id, state.ids.rectangle, rect, color, style, ui);
        match interaction {
            Interaction::Hover => {
                //  println!("hovering id:{:?}", id);
            }
            _ => {}
        }
        // Instantiate the image.
        let widget_image = show.image_id;
        let (x, y, w, h) = rect.x_y_w_h();
        let image = widget::Image::new(widget_image)
            .x_y(x, y)
            .top_right_with_margins_on(id, h * 0.2, w * 0.2)
            .w_h(w * 0.6, h * 0.6)
            .parent(id)
            .graphics_for(id);

        image.set(state.ids.image, ui);
        TimesClicked(interaction)
    }
    fn drag_area(&self, dim: Dimensions, style: &Style, theme: &Theme) -> Option<Rect> {
        Some(Rect::from_xy_dim([0.0, 0.0], dim))
    }
}

fn color_from_interaction(color: Color, interaction: Interaction, drag: &mut Drag) -> Color {
    match drag {
        &mut Drag::Selecting(_) => color.highlighted(),
        _ => {
            match interaction {
                Interaction::Idle => color,
                Interaction::Hover => color.highlighted(),
                Interaction::Press => color.clicked(),
                Interaction::Hold => color.highlighted(),
            }
        }
    }

}

fn interaction_and_times_triggered(button_id: widget::Id,
                                   drag: &mut Drag,
                                   ui: &UiCell)
                                   -> Interaction {
    let mut interaction = Interaction::Idle;
    for widget_event in ui.widget_input(button_id).events() {
        match widget_event {
            event::Widget::Press(press) => {
                match press.button {
                    event::Button::Mouse(input::MouseButton::Left, _) => {
                        let now = Instant::now();
                        match drag {
                            &mut Drag::Selecting(a) => {
                                if a.elapsed() >= Duration::from_secs(1) {
                                    interaction = Interaction::Hold;
                                    *drag = Drag::Terminate;
                                }
                            }
                            &mut Drag::None => {
                                *drag = Drag::Selecting(now);
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
                    _ => {
                        interaction = Interaction::Press;
                    }
                }
            }
            event::Widget::Release(release) => {
                if let event::Button::Mouse(input::MouseButton::Left, _) = release.button {
                    match drag {
                        &mut Drag::Selecting(a) => {
                            if a.elapsed() >= Duration::from_secs(1) {
                                *drag = Drag::Terminate;
                            } else {
                                *drag = Drag::None;
                            }
                            interaction = Interaction::Press;
                        }
                        _ => {}
                    }
                }
            }
            event::Widget::Drag(drag_event) if drag_event.button == input::MouseButton::Left => {
                match drag {
                    &mut Drag::Selecting(_) => {
                        let dim = ui.wh_of(button_id).unwrap();
                        if (drag_event.to.get_x().abs() > dim[0]) ||
                           (drag_event.to.get_y().abs() > dim[1]) {
                            *drag = Drag::None;
                            interaction = Interaction::Idle;
                        }
                    }
                    _ => {}
                }
            }
            /*   event::Widget::Click(click)=>match click.button{
                    interaction = Interaction::Press;
                },*/
            _ => {
                if let Drag::None = *drag {
                    interaction = Interaction::Hover;
                }
            }
        }
    }
    interaction
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

impl<S> Colorable for Button<S> {
    builder_method!(color { style.color = Some(Color) });
}

impl<S> Borderable for Button<S> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
