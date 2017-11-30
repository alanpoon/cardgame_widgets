use conrod::{self, widget, Positionable, Widget, Colorable, Rect, Sizeable};
use conrod::widget::envelope_editor::EnvelopePoint;
use custom_widget::image_hover::{Hoverable, ImageHover};

use std::fmt::Debug;
use std::marker::Send;
pub trait Arrangeable {
    fn arrangeable(self, bool) -> Self;
}

pub trait Selectable {
    fn selectable(self) -> Self;
}
pub enum ExitBy {
    Top,
    Bottom,
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + Debug,
          W: Widget + Arrangeable + Selectable,
          A: Hoverable
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// See the Style struct below.
    style: Style,
    values: &'a mut Vec<T>,
    widget_closure: Box<Fn(T) -> W>,
    item_width: f64,
    left_arrow: Option<A>,
    top_arrow: Option<A>,
    right_arrow: Option<A>,
    bottom_arrow: Option<A>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "20.0")]
    pub spacing: Option<f64>,
}

widget_ids! {
    struct Ids {
      rect,
      list_select,
      left_a,
      top_a,
      right_a,
      bottom_a,
    }
}

/// Represents the unique, cached state for our ArrangeList widget.
pub struct State {
    ids: Ids,
    selected: Option<usize>,
}

impl<'a, T, W, A> ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + Debug,
          W: Widget + Arrangeable + Selectable,
          A: Hoverable
{
    /// Create a button context to be built upon.
    pub fn new(values: &'a mut Vec<T>, widget_closure: Box<Fn(T) -> W>, item_width: f64) -> Self {
        ArrangeList {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            values: values,
            widget_closure: widget_closure,
            item_width: item_width,
            left_arrow: None,
            top_arrow: None,
            right_arrow: None,
            bottom_arrow: None,
        }
    }
    builder_methods!{
        pub spacing {style.spacing=Some(f64)}
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, T, W, A> Widget for ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + 'static + Debug,
          W: Widget + Arrangeable + Selectable,
          A: Hoverable
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when an element exited, otherwise `None`.
    type Event = (Option<T>, ExitBy, Option<widget::list::Scrollbar<widget::scroll::X>>);

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            selected: None,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, style, .. } = args;
        let spacing = self.style.spacing(&ui.theme());
        let mut top_left = rect.top_left();
        let mut btm_right = rect.bottom_right();
        if let Some(_) = self.left_arrow {
            top_left.set_x(rect.top_left().get_x() + spacing);
        }
        if let Some(_) = self.top_arrow {
            top_left.set_y(rect.top_left().get_y() - spacing);
        }
        if let Some(_) = self.right_arrow {
            btm_right.set_x(rect.bottom_right().get_x() - spacing);
        }
        if let Some(_) = self.bottom_arrow {
            btm_right.set_y(rect.bottom_right().get_y() + spacing);
        }
        let temp_rect = Rect::from_corners(top_left, btm_right);
        widget::Rectangle::fill(temp_rect.dim())
            .top_left_with_margins_on(id,rect.top_left().get_y()-top_left.get_y(),top_left.get_x()-rect.top_left().get_x())
            //.top_left_with_margins(top_left.get_y(),top_left.get_x())
            .graphics_for(id)
            .color(style.color(&ui.theme))
            .set(state.ids.rect, ui);

        let (mut events, scrollbar) = widget::ListSelect::single(self.values.len())
            .flow_right()
            .item_size(self.item_width)
            .scrollbar_next_to()
            .wh_of(state.ids.rect)
            .middle_of(state.ids.rect)
            .set(state.ids.list_select, ui);
        while let Some(event) = events.next(ui, |i| {
            let mut y = false;
            if let Some(_x) = state.selected {
                if _x == i {
                    y = true;
                }
            }
            y
        }) {
            use conrod::widget::list_select::Event;
            match event {
                // For the `Item` events we instantiate the `List`'s items.
                Event::Item(item) => {
                    let k_h = self.values.get(item.i).unwrap();
                    let mut widget = (*self.widget_closure)(k_h.clone());
                    if let Some(_s) = state.selected {
                        if item.i == _s {
                            widget = widget.selectable();
                        }
                        item.set(widget, ui);
                    }

                }
                Event::Selection(selected_id) => {
                    state.update(|state| state.selected = Some(selected_id));
                }
                event => println!("{:?}", &event),
            }
        }
        if let Some(_a) = self.left_arrow {
            let j = ImageHover::new(_a)
                .w_h(spacing, spacing)
                .left_from(state.ids.rect, 0.0)
                .set(state.ids.left_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if _s > 0 {
                        rearrange(_s, _s - 1, self.values);
                        state.update(|state| state.selected = Some(_s - 1));
                    }

                }
            }
        }
        let mut exit_elem: Option<T> = None;
        let mut exit_by: ExitBy = ExitBy::Top;
        if let Some(_a) = self.top_arrow {
            let j = ImageHover::new(_a)
                .w_h(spacing, spacing)
                .up_from(state.ids.rect, 0.0)
                .set(state.ids.top_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if self.values.len() > 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| state.selected = Some(_s - 1));
                    } else if self.values.len() == 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| state.selected = None);
                    }
                }
            }
        }
        if let Some(_a) = self.right_arrow {
            let j = ImageHover::new(_a)
                .w_h(spacing, spacing)
                .right_from(state.ids.rect, 0.0)
                .set(state.ids.right_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if _s < self.values.len() - 1 {
                        rearrange(_s, _s + 1, self.values);
                        state.update(|state| state.selected = Some(_s + 1));
                    }

                }
            }
        }
        if let Some(_a) = self.bottom_arrow {
            let j = ImageHover::new(_a)
                .w_h(spacing, spacing)
                .down_from(state.ids.rect, 0.0)
                .set(state.ids.right_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if self.values.len() > 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| state.selected = Some(_s - 1));
                        exit_by = ExitBy::Bottom;
                    } else if self.values.len() == 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| state.selected = None);
                        exit_by = ExitBy::Bottom;
                    }
                }
            }
        }
        (exit_elem, exit_by, scrollbar)
    }
}

fn remove_by_index<T: Clone>(c2: usize, hash: &mut Vec<T>) -> T {
    hash.remove(c2)
}

fn rearrange<T: Clone>(selected_i: usize, corrected_i: usize, hash: &mut Vec<T>) {
    let hash_c = hash.clone();
    for (_i, value) in hash.iter_mut().enumerate() {
        if _i == corrected_i {
            if let Some(v2) = hash_c.get(selected_i) {
                *value = v2.clone();
            }
        }
        if selected_i < corrected_i {
            //moved backward ____S__->__C
            if (_i < corrected_i) & (_i >= selected_i) {
                // ____S~~~~C;
                if let Some(v2) = hash_c.get(_i + 1) {
                    *value = v2.clone();
                }
            }
        } else if selected_i > corrected_i {
            //moved foward _____C__<-S
            if (_i <= selected_i) & (_i > corrected_i) {
                // ____C~~~S
                if let Some(v2) = hash_c.get(_i - 1) {
                    *value = v2.clone();
                }
            }
        }

    }

}
