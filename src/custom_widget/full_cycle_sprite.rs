use conrod_core::{widget, Positionable, Widget, Sizeable, Rect, image};
use sprite::{spriteable_rect, Spriteable};
use std::time::{Duration, Instant};

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct FullCycleSprite<H: Spriteable> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image: image::Id,
    pub sprite: H,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    ///the smaller the value, the slower it gets
    #[conrod(default = "0.5")]
    pub frame_rate: Option<f32>,
}

widget_ids! {
    struct Ids {
        bottle0,
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
    frame: f32,
    last_update: Instant,
}

impl<H> FullCycleSprite<H>
    where H: Spriteable
{
    /// Create a button context to be built upon.
    pub fn new(image: image::Id, sprite: H) -> Self {
        FullCycleSprite {
            image: image,
            sprite: sprite,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
    builder_methods!{
        pub frame_rate { style.frame_rate = Some(f32) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<H> Widget for FullCycleSprite<H>
    where H: Spriteable
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
        State {
            ids: Ids::new(id_gen),
            frame: 0.0,
            last_update: Instant::now(),
        }
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
        let frame_rate = self.style.frame_rate(ui.theme());
        let num = self.sprite.num_in_col() * self.sprite.num_in_row();
        let frame_c = state.frame.clone();
        let now = Instant::now();
        let _step = if now.clone().duration_since(state.last_update) < Duration::new(3, 0) {
            if frame_c <= num as f32 {
                state.update(|state| {
                                 state.frame += frame_rate;
                                 state.last_update = now;
                             });
                Some((frame_c.floor() as u16 % num) as f64)
            } else {
                state.update(|state| state.frame = 0.0);
                Some(0.0)
            }
        } else {
            state.update(|state| {
                             state.frame = 0.0;
                             state.last_update = now;
                         });
            Some(0.0)
        };

        let s = self.sprite;
        if let Some(_s) = _step {
            let r = spriteable_rect(s, _s);
            widget::Image::new(self.image)
                .source_rectangle(Rect::from_corners(r.0, r.1))
                .w_h(w, h)
                .middle_of(id)
                .parent(id)
                .graphics_for(id)
                .set(state.ids.bottle0, ui);

        }

        Some(())
    }
}
