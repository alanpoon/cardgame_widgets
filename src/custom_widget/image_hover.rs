use conrod::{self, widget, Positionable, Widget, Sizeable};
use conrod::widget::primitive::image::Image;
pub trait Hoverable {
    fn idle(&self) -> Image;
    fn hover(&self) -> Option<Image>;
    fn press(&self) -> Option<Image>;
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ImageHover<H: Hoverable> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image: H,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {}

widget_ids! {
    struct Ids {
        bottle0,
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
impl<H> ImageHover<H>
    where H: Hoverable
{
    /// Create a button context to be built upon.
    pub fn new(image: H) -> Self {
        ImageHover {
            image: image,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<H> Widget for ImageHover<H>
    where H: Hoverable
{
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
        let (interaction, _times_triggered) = interaction_and_times_triggered(id, ui);

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (_, _, w, h) = rect.x_y_w_h();
        let _image = match interaction {
            Interaction::Idle => self.image.idle(),
            Interaction::Hover => self.image.hover().unwrap_or(self.image.idle()),
            Interaction::Press => self.image.press().unwrap_or(self.image.idle()),
        };

        _image.w_h(w, h)
            .middle_of(id)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.bottle0, ui);

        TimesClicked(_times_triggered)
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
