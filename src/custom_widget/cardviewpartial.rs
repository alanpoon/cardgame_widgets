use conrod::{self, widget, Positionable, Widget, image, Sizeable};
use sprite::SpriteInfo;
use custom_widget::animated_button::AnimatedButton;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct CardViewPartial<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image: image::Id,
    card_style_t: usize,
    pub vec_style: Vec<SpriteInfo>,
    pub card_index: f64,
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
        bottle0,
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<'a> CardViewPartial<'a> {
    /// Create a button context to be built upon.
    pub fn new(image: image::Id,
               card_style_t: usize,
               vec_style: Vec<SpriteInfo>,
               card_index: f64,
               name: &'a str)
               -> Self {
        CardViewPartial {
            image: image,
            card_style_t: card_style_t,
            vec_style: vec_style,
            card_index: card_index,
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
impl<'a> Widget for CardViewPartial<'a> {
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
        let widget::UpdateArgs { id, state, rect, mut ui, .. } = args;
        let (interaction, times_triggered) = interaction_and_times_triggered(id, ui);

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (x, y, w, h) = rect.x_y_w_h();
        let wh_rect = match interaction {
            Interaction::Idle => (w, h),
            Interaction::Hover => (1.5 * w, 1.5 * h),
            Interaction::Press => (w, h),
        };

        let _style = self.vec_style[self.card_style_t];
        let te = _style.src_rect(self.card_index as f64);
        widget::Image::new(self.image)
            .source_rectangle(te)
            .w_h(wh_rect.0, wh_rect.1)
            .middle()
            .parent(id)
            .graphics_for(id)
            .set(state.ids.bottle0, ui);

        Some(())
    }
}

#[derive(Copy, Clone)]
enum Interaction {
    Idle,
    Hover,
    Press,
}

fn interaction_and_times_triggered(button_id: widget::Id,
                                   ui: &conrod::UiCell)
                                   -> (Interaction, u16) {
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
