use conrod_core::{self, widget, Positionable, Widget, image, Sizeable, Rect};
use sprite::{spriteable_rect, Spriteable};

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct CardViewPartial<'a, H>
    where H: Spriteable + Clone
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image: image::Id,
    card_style_t: usize,
    pub vec_sprite: Vec<H>,
    pub card_index: f64,
    pub name: &'a str,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod_core::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod_core::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod_core::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod_core::text::font::Id>>,
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

impl<'a, H> CardViewPartial<'a, H>
    where H: Spriteable + Clone
{
    /// Create a button context to be built upon.
    pub fn new(image: image::Id,
               card_style_t: usize,
               vec_sprite: Vec<H>,
               card_index: f64,
               name: &'a str)
               -> Self {
        CardViewPartial {
            image: image,
            card_style_t: card_style_t,
            vec_sprite: vec_sprite,
            card_index: card_index,
            name: name,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod_core::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, H> Widget for CardViewPartial<'a, H>
    where H: Spriteable + Clone
{
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
        let (interaction, _times_triggered) = interaction_and_times_triggered(id, ui);

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (_, _, w, h) = rect.x_y_w_h();
        let wh_rect = match interaction {
            Interaction::Idle => (w, h),
            Interaction::Hover => (1.5 * w, 1.5 * h),
            Interaction::Press => (w, h),
        };

        let _sprite = self.vec_sprite[self.card_style_t].clone();
        let r = spriteable_rect(_sprite, self.card_index as f64);
        widget::Image::new(self.image)
            .source_rectangle(Rect::from_corners(r.0, r.1))
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
                                   ui: &conrod_core::UiCell)
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
