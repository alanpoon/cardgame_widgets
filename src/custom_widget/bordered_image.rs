use conrod_core::{widget, Positionable, Widget, Sizeable, image, Color, Rect, Scalar};
use conrod_core::widget::Rectangle;
use conrod_core::UiCell;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct BorderedImage {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image_id: image::Id,
    pub src_rect: Option<Rect>,
    pub bordered: bool,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Width of the border surrounding the Image
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
}

widget_ids! {
    struct Ids {
        rect,
        image,
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}
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
impl BorderedImage {
    /// Create a button context to be built upon.
    pub fn new(image_id: image::Id) -> Self {
        BorderedImage {
            image_id: image_id,
            src_rect: None,
            common: widget::CommonBuilder::default(),
            bordered: false,
            style: Style::default(),
        }
    }
    builder_methods!{
        pub border { style.border = Some(Scalar) }
        pub border_color { style.border_color = Some(Color) }
    }
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }
    pub fn source_rectangle(mut self, _rect: Rect) -> Self {
        self.src_rect = Some(_rect);
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl Widget for BorderedImage {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        let (_interaction, times_triggered) = interaction_and_times_triggered(id, ui);
        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (_, _, w, h) = rect.x_y_w_h();
        let border = if self.bordered {
            self.style.border(ui.theme())
        } else {
            0.0
        };
        if self.bordered {
            let border_color = self.style.border_color(ui.theme());
            let _style = widget::line::Style {
                maybe_pattern: None,
                maybe_color: Some(border_color),
                maybe_thickness: Some(border),
                maybe_cap: None,
            };
            Rectangle::outline_styled([w, h],_style).middle_of(id)
            .parent(id)
            //.graphics_for(id)
            .set(state.ids.rect, ui);
        }

        let mut j = widget::Image::new(self.image_id).middle_of(id).padded_wh_of(id, border);
        if let Some(_src_rect) = self.src_rect {
            j = j.source_rectangle(_src_rect);
        }
        j.parent(id).graphics_for(id).set(state.ids.image, ui);
        TimesClicked(times_triggered)
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
#[derive(Copy, Clone)]
enum Interaction {
    Idle,
    Hover,
    Press,
}
pub trait Bordered {
    fn bordered(self) -> Self;
}
