use conrod::{widget, Positionable, Widget, Color, Colorable, Sizeable};
use text::get_font_size_hn;
use std::time::Duration;
use std::time::Instant;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct Notification<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub text: &'a str,
    pub start: Instant,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// The color of the Canvas' rectangle surface.
    #[conrod(default = "theme.background_color")]
    pub color: Option<Color>,
    /// The number of lines
    #[conrod(default = "3.0")]
    pub num_lines: Option<f64>,
    /// Duration
    #[conrod(default="Duration::new(4,0)")]
    pub duration: Option<Duration>,
}

widget_ids! {
    struct Ids {
        rect,
        text
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<'a> Notification<'a> {
    /// Create a button context to be built upon.
    pub fn new(text: &'a str, start: Instant) -> Self {
        Notification {
            text: text,
            start: start,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
    builder_methods!{
     pub num_lines { style.num_lines = Some(f64) }
     pub duration{style.duration = Some(Duration)}
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for Notification<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = ();

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

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        if self.start.elapsed() < self.style.duration(&ui.theme) {
            let (_, _, w, h) = rect.x_y_w_h();
            let color = self.style.color(&ui.theme);
            widget::Rectangle::fill_with([w, h], color)
                .middle_of(id)
                .parent(id)
                .set(state.ids.rect, ui);
            let n = self.style.num_lines(&ui.theme);
            let font_size = get_font_size_hn(h, n);
            widget::Text::new(self.text)
                .font_size(font_size)
                .middle_of(id)
                .wh_of(id)
                .parent(id)
                .color(color.plain_contrast())
                .set(state.ids.text, ui);
        }

    }
}
impl<'a> Colorable for Notification<'a> {
    builder_method!(color { style.color = Some(Color) });
}
