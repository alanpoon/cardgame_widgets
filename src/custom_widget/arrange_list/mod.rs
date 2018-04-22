use conrod::{self, widget, Positionable, Widget, Colorable, Sizeable, Color};
use conrod::widget::list::{Right,Fixed};
use conrod::UiCell;
use std::fmt::Debug;
use std::marker::Send;
pub use custom_widget::image_hover::{Hoverable, ImageHover, TimesClicked};
pub mod item;

pub use custom_widget::arrange_list::item::ItemWidget;
pub trait Arrangeable {
    fn selectable(self) -> Self;
}
pub trait WidgetMut<T>{
    fn set_mut<'a,'b>(self,widget::list::Item<Right,Fixed>,&'a mut UiCell<'b>)->T;
}
pub enum ExitBy {
    Top,
    Bottom,
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + Debug,
          W: WidgetMut<T> + Arrangeable,
          A: Hoverable
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// See the Style struct below.
    style: Style,
    values: &'a mut Vec<T>,
    widget_closure: Box<'a + Fn(T,&mut bool) -> W>,
    blow_up_closure: Box<'a + Fn(T) -> usize>,
    blow_up: &'a mut Option<usize>,
    show_selected: &'a mut Option<widget::Id>,
    item_width: f64,
    left_arrow: Option<A>,
    top_arrow: Option<A>,
    right_arrow: Option<A>,
    bottom_arrow: Option<A>,
    corner_arrow: Option<A>,
    keypad_bool:Option<bool>
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "40.0")]
    pub arrow_size: Option<f64>,
    #[conrod(default="10.0")]
    pub scrollbar_thickness: Option<f64>,
}

widget_ids! {
    struct Ids {
     test,
      rect,
      list_select,
      left_a,
      top_a,
      right_a,
      bottom_a,
      corner_a
    }
}

/// Represents the unique, cached state for our ArrangeList widget.
pub struct State {
    ids: Ids,
    selected: Option<usize>,
    s_widget_id: Option<widget::Id>,
}

impl<'a, T, W, A> ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + Debug,
          W: WidgetMut<T> + Arrangeable,
          A: Hoverable
{
    /// Create a button context to be built upon.
    pub fn new(values: &'a mut Vec<T>,
               show_selected: &'a mut Option<widget::Id>,
               blow_up: &'a mut Option<usize>,
               widget_closure: Box<'a+Fn(T,&mut bool) -> W>,
               blow_up_closure: Box<'a+Fn(T) -> usize>,
               item_width: f64)
               -> Self {
        ArrangeList {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            values: values,
            show_selected: show_selected,
            widget_closure: widget_closure,
            blow_up_closure: blow_up_closure,
            blow_up: blow_up,
            item_width: item_width,
            left_arrow: None,
            top_arrow: None,
            right_arrow: None,
            bottom_arrow: None,
            corner_arrow: None,
            keypad_bool:None
        }
    }
    builder_methods!{
        pub arrow_size {style.arrow_size=Some(f64)}
        pub scrollbar_thickness{style.scrollbar_thickness=Some(f64)}
    }
    pub fn left_arrow(mut self, _h: A) -> Self {
        self.left_arrow = Some(_h);
        self
    }
    pub fn top_arrow(mut self, _h: A) -> Self {
        self.top_arrow = Some(_h);
        self
    }
    pub fn right_arrow(mut self, _h: A) -> Self {
        self.right_arrow = Some(_h);
        self
    }
    pub fn bottom_arrow(mut self, _h: A) -> Self {
        self.bottom_arrow = Some(_h);
        self
    }
    pub fn corner_arrow(mut self, _h: A) -> Self {
        self.corner_arrow = Some(_h);
        self
    }
    pub fn keypad_bool(mut self,_h:Option(bool))-> Self{
        self.keypad_bool = Some(_h);
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, T, W, A> Widget for ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + 'static + Debug,
          W: WidgetMut<T> + Arrangeable,
          A: Hoverable
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when an element exited, otherwise `None`.,Selected_index
    type Event = (Option<T>, ExitBy, Option<widget::list::Scrollbar<widget::scroll::X>>);

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            selected: None,
            s_widget_id: None,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, style, .. } = args;
        let arrow_size = self.style.arrow_size(&ui.theme());
        let values_clone = self.values.clone();
        widget::Rectangle::fill(rect.dim())
            .middle_of(id)
            .graphics_for(id)
            .color(style.color(&ui.theme))
            .set(state.ids.rect, ui);
        if let &mut Some(_id) = self.show_selected {
            if _id != id {
                state.update(|state| {
                                 state.selected = None;
                                 state.s_widget_id = None;
                             });
            }
        }
        if let Some(_s) = state.selected {
            if _s >= values_clone.len() {
                state.update(|state| {
                    if values_clone.len() == 0 {
                        state.selected = None;
                        state.s_widget_id = None;
                    } else {
                        state.selected = Some(values_clone.len() - 1);
                    };
                });
            }
        }
        let keypad_bools = vec![self.keypad_bool;values_clone.len()];
        let (mut events, scrollbar) = widget::ListSelect::single(values_clone.len())
            .flow_right()
            .item_size(self.item_width)
            .scrollbar_thickness(self.style.scrollbar_thickness(&ui.theme))
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
                    let k_h_c= self.values.get(item.i).unwrap().clone();
                    let keypad_bool_ind = keypad_bools.get_mut(item.i).unwrap();
                    let mut widget = (*self.widget_closure)(k_h_c,keypad_bool_ind);
                    if let Some(_s) = state.selected {
                        if item.i == _s {
                            widget = widget.selectable();
                            state.update(|state| state.s_widget_id = Some(item.widget_id));
                        }
                    }                    
                    let k_h_m = self.values.get_mut(item.i).unwrap();
                    *k_h_m = widget.set_mut(item,ui);
                }
                Event::Selection(selected_id) => {
                    *self.show_selected = Some(id);
                    state.update(|state| state.selected = Some(selected_id));
                }
                _ => {}
            }
        }
        if let (Some(_a), Some(_s_id)) = (self.left_arrow, state.s_widget_id) {
            let j = ImageHover::new(_a)
                .w_h(arrow_size, arrow_size)
                .align_middle_y_of(_s_id)
                .left_from(_s_id, -arrow_size)
                .set(state.ids.left_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if _s > 0 {
                        rearrange(_s, _s - 1, self.values);
                        state.update(|state| state.selected = Some(_s - 1));
                    } else {
                        state.update(|state| {
                                         state.selected = None;
                                         state.s_widget_id = None;
                                     });
                    }

                }
            }
        }
        let mut exit_elem: Option<T> = None;
        let mut exit_by: ExitBy = ExitBy::Top;
        if let (Some(_a), Some(_s_id)) = (self.top_arrow, state.s_widget_id) {
            let j = ImageHover::new(_a)
                .w_h(arrow_size, arrow_size)
                .align_middle_x_of(_s_id)
                .up_from(_s_id, -arrow_size)
                .set(state.ids.top_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if values_clone.len() > 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| {
                                         state.selected = Some(_s - 1);
                                         state.s_widget_id = None;
                                     });
                    } else if values_clone.len() == 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| {
                                         state.selected = None;
                                         state.s_widget_id = None;
                                     });
                    }

                }
            }
        }
        if let (Some(_a), Some(_s_id)) = (self.right_arrow, state.s_widget_id) {
            let j = ImageHover::new(_a)
                .w_h(arrow_size, arrow_size)
                .align_middle_y_of(_s_id)
                .right_from(_s_id, -arrow_size)
                .set(state.ids.right_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if _s < values_clone.len() - 1 {
                        rearrange(_s, _s + 1,self.values);
                        state.update(|state| state.selected = Some(_s + 1));
                    } else {
                        state.update(|state| {
                                         state.selected = None;
                                         state.s_widget_id = None;
                                     });
                    }

                }
            }
        }
        if let (Some(_a), Some(_s_id)) = (self.bottom_arrow, state.s_widget_id) {
            let j = ImageHover::new(_a)
                .w_h(arrow_size, arrow_size)
                .align_middle_x_of(_s_id)
                .down_from(_s_id, -arrow_size)
                .set(state.ids.bottom_a, ui);
            if let Some(_s) = state.selected {
                for _c in j {
                    if values_clone.len() > 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        exit_by = ExitBy::Bottom;
                        state.update(|state| {
                                         state.selected = Some(_s - 1);
                                         state.s_widget_id = None;
                                     });
                    } else if values_clone.len() == 1 {
                        exit_elem = Some(remove_by_index(_s, self.values));
                        state.update(|state| {
                                         state.selected = None;
                                         state.s_widget_id = None;
                                     });
                        exit_by = ExitBy::Bottom;
                    }
                }
            }
        }
        if let (Some(_a), Some(_s_id)) = (self.corner_arrow, state.s_widget_id) {
            let j = ImageHover::new(_a)
                .w_h(arrow_size, arrow_size)
                .top_right_with_margin_on(_s_id, -2.0)
                .set(state.ids.corner_a, ui);
            if let Some(_s) = state.selected {
                if let &mut Some(_b) = self.blow_up {
                    let k_h = values_clone.get(_s).unwrap();
                    let k = (*self.blow_up_closure)(k_h.clone());
                    if _b != k {
                        *self.blow_up = Some(k);
                    }
                }
                for _c in j {
                    if let &mut Some(_b) = self.blow_up {
                        let k_h = values_clone.get(_s).unwrap();
                        let k = (*self.blow_up_closure)(k_h.clone());
                        if _b == k {
                            *self.blow_up = None;
                        } else {
                            *self.blow_up = Some(k);
                        }

                    } else {
                        let k_h = values_clone.get(_s).unwrap();
                        let k = (*self.blow_up_closure)(k_h.clone());
                        *self.blow_up = Some(k);
                    }
                }
            }
        }
        let keypad_bool_new=false;
        let keypad_true_len = keypad_bools.iter().filter(|x|{}).collect().len();
        if (keypad_true_len>0){
            keypad_bool_new=true;
        }
        (exit_elem, exit_by, scrollbar,keypad_bool_new)
    }
}
impl<'a, T, W, A> Colorable for ArrangeList<'a, T, W, A>
    where T: Clone + Send + 'a + 'static + Debug,
          W: WidgetMut<T> + Arrangeable,
          A: Hoverable
{
    builder_method!(color { style.color = Some(Color) });
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
