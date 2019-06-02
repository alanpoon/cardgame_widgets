use conrod_core::{widget, Positionable, Widget, Sizeable, UiCell};
use conrod_core::widget::primitive::image::Image;
use std::fmt::Debug;
use std::marker::Send;
#[derive(WidgetCommon)]
pub struct Shuffle<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget
{
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// See the Style struct below.
    style: Style,
    values: &'a Vec<T>,
    widget_closure: Box<Fn(T) -> W>,
    pub back_card: Image,
    pub give_out: Option<Vec<usize>>,
}
#[derive(Debug)]
enum AniState {
    Waitthen,
    Keep(u16),
    Backview,
    Shuffle(u16),
    Giveout(u16),
    Reset,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "[220.0,260.0]")]
    pub image_dim: Option<[f64; 2]>,
    /// The number of frames to keep one card
    #[conrod(default="60")]
    pub close_frame_rate: Option<u16>,
}
widget_ids! {
    struct Ids {
        listview,
        items[],
        backview,
        backview2
    }
}
/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
    frame: u16,
    num_closed: i8,
}

impl<'a, T, W> Shuffle<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget
{
    /// Create a button context to be built upon.
    pub fn new(values: &'a Vec<T>, widget_closure: Box<Fn(T) -> W>, back_card: Image) -> Self {
        Shuffle {
            values: values,
            widget_closure: widget_closure,
            back_card: back_card,
            give_out: None,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
    builder_methods!{
        pub image_dim { style.image_dim = Some([f64;2]) }
        pub close_frame_rate { style.close_frame_rate = Some(u16) }
    }
    pub fn give_out(mut self, _giveout: Vec<usize>) -> Self {
        self.give_out = Some(_giveout);
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, T, W> Widget for Shuffle<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = bool;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            frame: 0,
            num_closed: 0,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, ui, .. } = args;
        let image_dim = self.style.image_dim(ui.theme());
        let len = self.values.len();
        if state.ids.items.len() < len {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.items.resize(len, id_gen));
        }
        let close_frame_rate = self.style.close_frame_rate(ui.theme()); //frames to slot the last card to the back of its previous
        let item_c = state.ids.items.clone();
        let mut item_iter = item_c.iter().enumerate();
        let mut value_iter = self.values.iter();
        let mut num_closed = state.num_closed.clone();
        let step_state =
            if state.frame <= close_frame_rate {
                AniState::Waitthen
            } else if state.frame <= close_frame_rate * (self.values.len() + 1) as u16 {
                AniState::Keep(state.frame % close_frame_rate)
            } else if state.frame <= close_frame_rate * (self.values.len() + 2) as u16 {
                AniState::Backview
            } else if state.frame <= close_frame_rate * (self.values.len() + 3) as u16 {
                AniState::Shuffle(state.frame % close_frame_rate)
            } else if state.frame <= close_frame_rate * (self.values.len() * 2 + 4) as u16 {
                if let Some(_) = self.give_out {
                    AniState::Giveout(state.frame % close_frame_rate)
                } else {
                    AniState::Reset
                }
            } else {
                AniState::Reset
            };


        match step_state {
            AniState::Waitthen => {
                while let (Some((_i, ref _sym)), Some(ref _value)) =
                    (item_iter.next(), value_iter.next()) {
                    let k = _i as f64;
                    (*self.widget_closure)(_value.clone().clone())
                        .w_h(image_dim[0], image_dim[1])
                        .mid_left_with_margin_on(id, k * image_dim[0])
                        .set(_sym.clone().clone(), ui);
                }
            }
            AniState::Keep(_step) => {
                while let (Some((_i, ref _sym)), Some(ref _value)) =
                    (item_iter.next(), value_iter.next()) {
                    let k = _i as f64;
                    let _widget = (*self.widget_closure)(_value.clone().clone());
                    render_movable(k,
                                   len,
                                   state.frame.clone() - close_frame_rate,
                                   close_frame_rate,
                                   _step,
                                   num_closed,
                                   image_dim,
                                   id,
                                   _widget,
                                   Direction::Left,
                                   _sym.clone().clone(),
                                   ui);
                }
                let sign: i8 = -1;
                if (_step == 0) & (num_closed < len as i8) & (state.frame != 0) {
                    num_closed = num_closed + sign * (-1);
                }
            }
            AniState::Backview => {
                self.back_card
                    .w_h(image_dim[0], image_dim[1])
                    .mid_left_with_margin_on(id, 0.0)
                    .set(state.ids.backview, ui);
            }
            AniState::Shuffle(_step) => {
                self.back_card
                    .w_h(image_dim[0], image_dim[1])
                    .mid_left_with_margin_on(id, 0.0)
                    .set(state.ids.backview, ui);
                if _step % 5 < 2 {
                    self.back_card
                        .w_h(image_dim[0], image_dim[1])
                        .mid_top_with_margin_on(state.ids.backview, image_dim[1] * 0.3)
                        .set(state.ids.backview2, ui)
                }
            }
            AniState::Giveout(_step) => {
                let give_out_c = self.give_out.clone().unwrap();
                let mut give_out_iter = give_out_c.iter();
                while let (Some((_i, ref _sym)), Some(&_giveout)) =
                    (item_iter.next(), give_out_iter.next()) {
                    if let Some(ref _value) = self.values.get(_giveout) {
                        let k = _i as f64;
                        let _widget = (*self.widget_closure)(_value.clone().clone());
                        render_movable(k,
                                       len,
                                       state.frame.clone() -
                                       close_frame_rate * (self.values.len() as u16 + 3),
                                       close_frame_rate,
                                       _step,
                                       num_closed,
                                       image_dim,
                                       id,
                                       _widget,
                                       Direction::Right,
                                       _sym.clone().clone(),
                                       ui);
                    }
                }
                let sign: i8 = 1;
                if (_step == 0) & (num_closed > 0) & (state.frame != 0) {
                    num_closed = num_closed + sign * (-1);
                }
            }
            AniState::Reset => {}
        }

        state.update(|state| state.num_closed = num_closed);
        state.update(|state| state.frame += 1);


        if let AniState::Reset = step_state {
            state.update(|state| {
                             state.frame = 0;
                             state.num_closed = 0;
                         });
            false
        } else {
            true
        }
    }
}
enum Direction {
    Right,
    Left,
}
fn render_movable<T: Widget>(k: f64,
                             len: usize,
                             frame: u16,
                             close_frame_rate: u16,
                             _step: u16,
                             num_closed: i8,
                             image_dim: [f64; 2],
                             id: widget::Id,
                             _widget: T,
                             direction: Direction,
                             _sym: widget::id::Id,
                             ui: &mut UiCell) {
    let (sign, image_to_move, less_than_show): (f64, f64, f64) = if let Direction::Left =
        direction {
        (image_dim[0] * (1.0 / close_frame_rate as f64) * _step as f64,
         (len as f64 - 1.0) - (frame as f64 / close_frame_rate as f64).floor() as f64,
         (len as i8 - 1 - num_closed) as f64)
    } else {
        (image_dim[0] - (1.0 / close_frame_rate as f64) * _step as f64,
         (frame as f64 / close_frame_rate as f64).floor(),
         (len as i8 - num_closed) as f64)
    };
    if k == image_to_move {
        let dis = if k > 0.0 {
            k * image_dim[0] - sign
        } else {
            0.0
        };
        _widget.w_h(image_dim[0], image_dim[1]).mid_left_with_margin_on(id, dis).set(_sym, ui);
    } else if k < less_than_show {
        _widget.w_h(image_dim[0], image_dim[1])
            .mid_left_with_margin_on(id, k * image_dim[0])
            .set(_sym, ui);
    }


}
