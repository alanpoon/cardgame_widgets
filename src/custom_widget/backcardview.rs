use conrod::{self, widget, Positionable, Widget, Sizeable, color, text, image};
use sprite::SpriteInfo;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct BackCardView<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub card_image: image::Id,
    pub card_index: f64,
    pub card_sprite: SpriteInfo,
    pub name: &'a str,
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
        frame,
        cardbase,
        player_name,
    }
}

/// Represents the unique, cached state for our BackCardView widget.
pub struct State {
    ids: Ids,
}

impl<'a> BackCardView<'a> {
    /// Create a button context to be built upon.
    pub fn new(card_image: image::Id,
               card_index: f64,
               card_sprite: SpriteInfo,
               name: &'a str)
               -> Self {
        BackCardView {
            card_image: card_image,
            card_index: card_index,
            card_sprite: card_sprite,
            name: name,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for BackCardView<'a> {
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
        let widget::UpdateArgs { state, rect, mut ui, .. } = args;

        let (x, _, _, h) = rect.x_y_w_h();

        widget::Image::new(self.card_image)
            .source_rectangle(self.card_sprite.src_rect(self.card_index))
            .mid_top()
            .h(0.7 * h)
            .padded_w_of(state.ids.frame, 5.0)
            .set(state.ids.cardbase, ui);

        let mut f1 = 16 as u32;
        let mut l1 = 2.5;

        while text::height(1, f1, l1) > 0.3 * h {
            f1 -= 1;
            l1 -= 0.2;
        }
        widget::Text::new(self.name)
            .font_size(f1)
            .down_from(state.ids.cardbase, 0.0)
            .padded_w_of(state.ids.cardbase, 10.0)
            .h(0.3 * h)
            .line_spacing(l1)
            .set(state.ids.player_name, ui);

        Some(())
    }
}
