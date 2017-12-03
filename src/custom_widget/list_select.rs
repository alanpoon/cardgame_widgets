use conrod::{self, widget, Colorable, Labelable, Positionable, Widget, image, Sizeable, Rect};

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ListSelect<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image_ids: Vec<ListItem>,
    /// Optional label string for the button.
    maybe_label: Option<&'a str>,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

widget_ids! {
    struct Ids {
        circle1,
        circle2,
        circle3,
        symbols[],
        text,
        rect,
    }
}

/// Represents the unique, cached state for our ListSelect widget.
pub struct State {
    ids: Ids,
}
pub enum ListItem {
    IMAGE(image::Id, Rect),
    NUM(i16),
    BRACKET(i16),
    None,
}
impl<'a> ListSelect<'a> {
    /// Create a button context to be built upon.
    pub fn new(image_ids: Vec<ListItem>) -> Self {
        ListSelect {
            image_ids: image_ids,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            maybe_label: None,
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
    /// other Conrod configs, this returns self for chainability. Allow dead code
    /// because we never call this in the example.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for ListSelect<'a> {
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
        let widget::UpdateArgs { id, state, rect, ui, style, .. } = args;
        let num = self.image_ids.len();
        if state.ids.symbols.len() < num {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.symbols.resize(num, id_gen));
        }
        let (color, event) = {
            let input = ui.widget_input(id);
            let color = style.color(&ui.theme);
            // If the button was clicked, produce `Some` event.
            let event = input.clicks()
                .left()
                .next()
                .map(|_| ());
            (color, event)
        };

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (x, y, w, h) = rect.x_y_w_h();
        widget::Rectangle::fill([w, h])
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.rect, ui);
        for it in self.image_ids.iter().zip(state.ids
                                                .symbols
                                                .iter()
                                                .enumerate()) {
            let (_id, (counter, &sym)) = it;
            match _id {
                &ListItem::IMAGE(im, sr) => {
                    widget::Image::new(im)
                        .source_rectangle(sr)
                        .middle_of(id)
                        .x_y(x + 0.3 * w + (counter as f64) * h, y)
                        .w_h(h, h)
                        .parent(id)
                        .set(sym, ui);
                }
                &ListItem::NUM(_iw) => {
                    let k = _iw.to_string();
                    widget::Text::new(&k)
                        .middle_of(id)
                        .x_y(x + 0.3 * w + (counter as f64) * h, y)
                        .w_h(h, h)
                        .parent(id)
                        .set(sym, ui);
                }
                &ListItem::BRACKET(_iw) => {
                    let f = format!("({})", _iw);
                    widget::Text::new(&f)
                        .middle_of(id)
                        .x_y(x + 0.3 * w + (counter as f64) * h, y)
                        .w_h(h, h)
                        .parent(id)
                        .set(sym, ui);
                }
                _ => {}
            }

        }

        // Now we'll instantiate our label using the **Text** widget.
        if let Some(ref label) = self.maybe_label {
            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::Text::new(label)
                .and_then(font_id, widget::Text::font_id)
                .mid_left_of(id)
                .font_size(font_size)
                .graphics_for(id)
                .color(label_color)
                .set(state.ids.text, ui);
        }

        event
    }
}

/// Provide the chainable label(), label_color(), and label_font_size()
/// configuration methods.
impl<'a> Labelable<'a> for ListSelect<'a> {
    fn label(mut self, text: &'a str) -> Self {
        self.maybe_label = Some(text);
        self
    }
    fn label_color(mut self, color: conrod::Color) -> Self {
        self.style.label_color = Some(color);
        self
    }
    fn label_font_size(mut self, size: conrod::FontSize) -> Self {
        self.style.label_font_size = Some(size);
        self
    }
}

impl<'a> Colorable for ListSelect<'a> {
    fn color(mut self, color: conrod::Color) -> Self {
        self.style.color = Some(color);
        self
    }
}
