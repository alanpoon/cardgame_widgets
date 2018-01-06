use conrod::{widget, Positionable, Widget, Scalar, FontSize, text, Color, Labelable, Colorable,
             color};
use conrod::widget::primitive::line;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ProgressBar<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    maybe_label: Option<&'a str>,
    len: usize,
    num_asset: usize,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    ///color
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    #[conrod(default = "2.0")]
    pub border: Option<Scalar>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The font size of the AnimatedButton's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<FontSize>,
    /// The ID of the font used to display the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<text::font::Id>>,
}

widget_ids! {
    struct Ids {
        outline,
        progressbar,
        loadingtxt,
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<'a> ProgressBar<'a> {
    /// Create a button context to be built upon.
    pub fn new(num_asset: usize, len: usize) -> Self {
        ProgressBar {
            maybe_label: None,
            len: len,
            num_asset: num_asset,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
    builder_methods!{
        pub border { style.border = Some(Scalar) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for ProgressBar<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

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
        let (_, _, w, h) = rect.x_y_w_h();
        // outer
        let line_style = line::Style::solid().color(color::BLACK);
        widget::Rectangle::outline_styled([w, h / 2.0], line_style).set(state.ids.outline, ui);
        let w_p = (self.num_asset as f64 / self.len as f64);
        let border = self.style.border(&ui.theme);
        widget::Rectangle::fill_with([w_p * (w - border * 2.0), h / 2.0 - border * 2.0],
                                     color::LIGHT_GREEN)
                .top_left_with_margin_on(state.ids.outline, border)
                .set(state.ids.progressbar, ui);
        if let Some(_txt) = self.maybe_label {
            let color = self.style.label_color(&ui.theme);
            let font_size = self.style.label_font_size(&ui.theme);
            widget::Text::new(_txt)
                .down_from(state.ids.outline, 0.0)
                .font_size(font_size)
                .color(color)
                .set(state.ids.loadingtxt, ui);
        }

        Some(())
    }
}
impl<'a> Labelable<'a> for ProgressBar<'a> {
    builder_methods!{
        label { maybe_label = Some(&'a str) }
        label_color { style.label_color = Some(Color) }
        label_font_size { style.label_font_size = Some(FontSize) }
    }
}
