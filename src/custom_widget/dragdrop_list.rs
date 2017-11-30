use conrod::{self, widget, Positionable, Widget, Colorable,Color,Sizeable};
use std;
use conrod::position::Scalar;
use std::fmt::Debug;
use std::marker::Send;
pub trait Draggable {
    fn draggable(self, bool) -> Self;
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct DragDropList<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget + Draggable,
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// See the Style struct below.
    style:  widget::list::Style,
    values: &'a mut Vec<T>,
    widget_closure: Box<Fn(T) -> W>,
    item_size:widget::list::Fixed,
    item_instantiation: widget::list::ItemInstantiation,
    exit_id: Option<widget::Id>,
    color: Option<conrod::Color>,
}

widget_ids! {
    struct Ids {
      list,
      items[],
      rect
    }
}

/// Represents the unique, cached state for our DragDropList widget.
pub struct State<T> {
    ids: Ids,
    temp: Vec<(Option<widget::Id>, T)>,
    last_release: Option<std::time::Instant>,
    mouse_point: Option<(usize, conrod::position::Point)>,
}


impl<'a, T, W> DragDropList<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget + Draggable
{
    /// Create a button context to be built upon.
    pub fn new(values: &'a mut Vec<T>, widget_closure: Box<Fn(T) -> W>, item_size: Scalar) -> Self {
        DragDropList {
            common: widget::CommonBuilder::default(),
            style: widget::list::Style::default(),
            values: values,
            widget_closure: widget_closure,
            item_size: widget::list::Fixed{length:item_size},
            item_instantiation: widget::list::ItemInstantiation::OnlyVisible,
            color:None,
            exit_id:None
        }
    }
   
    pub fn exit_id(mut self,id:widget::Id)->Self{
        self.exit_id = Some(id);
        self
    }
    pub fn instantiate_all_items(mut self) -> Self {
        self.item_instantiation = widget::list::ItemInstantiation::All;
        self
    }
    pub fn item_size(mut self,length:Scalar)->Self{
        self.item_size = widget::list::Fixed { length: length };
        self
    }
    pub fn scrollbar_next_to(mut self) -> Self {
        self.style.scrollbar_position = Some(Some(widget::list::ScrollbarPosition::NextTo));
        self
    }

    /// Specifies that the `List` should be scrollable and should provide a `Scrollbar` that hovers
    /// above the right edge of the items and automatically hides when the user is not scrolling.
    pub fn scrollbar_on_top(mut self) -> Self {
        self.style.scrollbar_position = Some(Some(widget::list::ScrollbarPosition::OnTop));
        self
    }

    /// The width of the `Scrollbar`.
    pub fn scrollbar_thickness(mut self, w: Scalar) -> Self {
        self.style.scrollbar_thickness = Some(Some(w));
        self
    }

    /// The color of the `Scrollbar`.
    pub fn scrollbar_color(mut self, color: Color) -> Self {
        self.style.scrollbar_color = Some(color);
        self
    }

}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, T, W> Widget for DragDropList<'a, T, W>
    where T: Clone + Send + 'a + 'static + Debug,
          W: Widget + Draggable
{
    /// The State struct that we defined above.
    type State = State<T>;
    /// The Style struct that we defined using from wigetList.
    type Style = widget::list::Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when an element exited, otherwise `None`.
    type Event = (Option<T>, Option<widget::list::Scrollbar<conrod::scroll::X>>);

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        let now = std::time::Instant::now();
        State {
            ids: Ids::new(id_gen),
            temp: vec![],
            last_release: Some(now),
            mouse_point: None,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, style, .. } = args;
        let w = rect.w();
        let h = rect.h();

        if state.ids.items.len() < self.values.len() {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.items.resize(self.values.len(), id_gen));
        }
        let mut list = widget::List::<widget::list::Right, _>::from_item_size(self.values.len(),
                                                                              self.item_size);
        let scrollbar_position = style.scrollbar_position(&ui.theme);
        list = match scrollbar_position {
            Some(widget::list::ScrollbarPosition::OnTop) => list.scrollbar_on_top(),
            Some(widget::list::ScrollbarPosition::NextTo) => list.scrollbar_next_to(),
            None => list,
        };
        list.item_instantiation = self.item_instantiation;
        list.style = style.clone();
        
        let value_c = self.values.clone();
       if let Some(_color) = self.color{
          widget::Rectangle::fill([w, h])
            .middle_of(id)
            .graphics_for(id)
            .color(_color)
            .set(state.ids.rect, ui);
       } else{
        widget::Rectangle::fill([w, h])
            .middle_of(id)
            .graphics_for(id)
            .set(state.ids.rect, ui);
       }
       let (mut items, scrollbar) = list.middle_of(id).wh_of(id).set(state.ids.list, ui);
        if state.temp.len() < value_c.len() {
            for _i in state.temp.len()..value_c.len() {
                if let Some(_v) = value_c.get(_i) {
                    state.update(|state| state.temp.push((None, _v.clone())));
                }
            }
        }
     
        let mut c = 0;

        let mut values_c_iter = value_c.iter();
        if let Some(_) = state.last_release {
            while let (Some(item), Some(k_h)) =
                (items.next(ui), values_c_iter.next()) {
                let widget = (*self.widget_closure)(k_h.clone());
                 item.set(widget,  ui);
                state.update(|state| {
                                 if let Some(a) = state.temp.get_mut(c) {
                                     *a = (Some(item.widget_id), k_h.clone());
                                 }
                                 state.last_release = None;
                             });

                c += 1;
            }

        } else {
            while let (Some(item), Some(k_h)) =
                (items.next(ui), values_c_iter.next()) {
                let widget = (*self.widget_closure)(k_h.clone());
                item.set(widget.draggable(true), ui);
                c += 1;
            }
        }
        let mut c2 = 0;
        let mut mouse_down = false;
        {
            let state_temp_c = state.temp.clone();
            let mut state_temp_iter = state_temp_c.iter();
            while let Some(&(Some(_j_id), _)) = state_temp_iter.next() {

                if let Some(mouse) = ui.widget_input(_j_id).mouse() {
                    if mouse.buttons.left().is_down() {
                        mouse_down = true;
                        state.update(|state| { state.mouse_point = Some((c2, mouse.abs_xy())); });
                    } else if mouse.buttons.left().is_up() {
                        mouse_down = false;
                        let now = std::time::Instant::now();
                        if let &None = &state.last_release {
                            state.update(|state| state.last_release = Some(now));
                        }
                    }
                }
                c2 += 1;
            }
        }
        let mut exit_id = None;
        if !mouse_down {
            if let Some((c2, m_point)) = state.mouse_point {
                let mut _c = 0;
                for _j in state.temp.iter() {
                    if _c == c2 {
                        _c += 1;
                        continue;
                    }
                    if let &(Some(p_widget), _) = _j {
                        let _p_rect = ui.rect_of(p_widget).unwrap();
                        if _p_rect.is_over(m_point) {
                            break;
                        }
                        _c += 1;
                    }

                }
                let mut rearrange_bool = false;
                if let Some(exit_rect) = self.exit_id {
                    if ui.rect_of(exit_rect).unwrap().is_over(m_point) {
                        rearrange_bool = false;
                        state.update(|state| {
                                         exit_id = Some(remove_by_index(c2, &mut state.temp));
                                     });
                    } else if ui.rect_of(state.ids.rect).unwrap().is_over(m_point) {
                        rearrange_bool = true;
                    }
                } else if !ui.rect_of(state.ids.rect).unwrap().is_over(m_point) {
                    rearrange_bool = false;
                    state.update(|state| { remove_by_index(c2, &mut state.temp); });
                } else {
                    rearrange_bool = true;
                }
                if rearrange_bool {
                    let len_of_some = state.temp.len();
                    let _k = if _c >= len_of_some { c2 } else { _c };
                    if _k != c2 {
                        state.update(|state| { rearrange(c2, _k, &mut state.temp); });
                    }
                }
                *self.values = state.temp
                    .iter()
                    .map(|&(_, ref value)| value.clone())
                    .collect::<Vec<T>>();
                state.update(|state| { state.mouse_point = None; });

            }
        }
        (exit_id, scrollbar)
    }
}
impl<'a, T, W> Colorable for DragDropList<'a, T, W>
    where T: Clone + Send + 'a + 'static + Debug,
          W: Widget + Draggable
          
{
    fn color(mut self, color: conrod::Color) -> Self {
        self.color = Some(color);
        self
    }
}
fn remove_by_index<T: Clone>(c2: usize, hash: &mut Vec<(Option<widget::Id>, T)>) -> T {
    hash.remove(c2).1
}
fn rearrange<T: Clone>(selected_i: usize,
                       corrected_i: usize,
                       hash: &mut Vec<(Option<widget::Id>, T)>) {
    let hash_c = hash.clone();
    for (_i, ref mut value) in hash.iter_mut().enumerate() {
        if _i == corrected_i {
            if let Some(v2) = hash_c.get(selected_i) {
                value.1 = v2.1.clone();
            }
        }
        if selected_i < corrected_i {
            //moved backward ____S__->__C
            if (_i < corrected_i) & (_i >= selected_i) {
                // ____S~~~~C;
                if let Some(v2) = hash_c.get(_i + 1) {
                    value.1 = v2.1.clone();
                }
            }
        } else if selected_i > corrected_i {
            //moved foward _____C__<-S
            if (_i <= selected_i) & (_i > corrected_i) {
                // ____C~~~S
                if let Some(v2) = hash_c.get(_i - 1) {
                    value.1 = v2.1.clone();
                }
            }
        }

    }

}
