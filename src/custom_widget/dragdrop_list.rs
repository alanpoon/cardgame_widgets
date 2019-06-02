use conrod_core::{self, widget, Positionable, Widget, Ui, UiCell, Colorable};
use std;
use conrod_core::position::Scalar;
use std::fmt::Debug;
use std::marker::Send;
pub trait Draggable {
    fn draggable(self, bool) -> Self;
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct DragDropList<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget + Draggable
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
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod_core::Color>,
    #[conrod(default="None")]
    pub exit_id: Option<Option<widget::Id>>,
}

widget_ids! {
    struct Ids {
      items[],
      rect
    }
}

/// Represents the unique, cached state for our DragDropList widget.
pub struct State<T> {
    ids: Ids,
    temp: Vec<(Option<widget::Id>, T)>,
    last_release: Option<std::time::Instant>,
    mouse_point: Option<(usize, conrod_core::position::Point)>,
}
/// The data necessary for instantiating a single item within a `List`.
#[derive( Debug)]
pub struct Item<'a> {
    pub i: usize,
    pub total_w: f64,
    pub acc_w: &'a mut f64,
    /// The id generated for the widget.
    pub widget_id: widget::Id,
    pub last_id: Option<widget::Id>,
    pub parent_id: widget::Id,
    pub first_left_id: &'a mut Option<widget::Id>,
}
impl<'a> Item<'a> {
    /// Sets the given widget as the widget to use for the item.
    ///
    /// Sets the:
    /// - position of the widget.
    /// - dimensions of the widget.
    /// - parent of the widget.
    /// - and finally sets the widget within the `Ui`.
    pub fn set<W>(self, widget: W, width: Scalar, ui: &mut UiCell) -> widget::Id
        where W: Widget
    {
        let Item { total_w, widget_id, last_id, parent_id, .. } = self;
        let acc_w_c = self.acc_w.clone();
        let first_left_id_c = self.first_left_id.clone();

        if acc_w_c + width > total_w {
            *(self.acc_w) = width;
            widget.and(|w| down_position_item(w, first_left_id_c, parent_id, 0.0))
                .and(|w| {
                         *(self.first_left_id) = Some(widget_id);
                         w
                     })
                .set(widget_id, ui);


        } else {
            *(self.acc_w) = acc_w_c + width;
            if let None = last_id {
                *(self.first_left_id) = Some(widget_id);
            }
            widget.and(|w| right_position_item(w, last_id, parent_id, 0.0)).set(widget_id, ui);
        }
        widget_id
    }
}
fn down_position_item<W>(widget: W,
                         last_id: Option<widget::Id>,
                         parent_id: widget::Id,
                         first_item_margin: Scalar)
                         -> W
    where W: Widget
{
    match last_id {
        None => {
            widget.mid_top_with_margin_on(parent_id, first_item_margin).align_left_of(parent_id)
        }
        Some(id) => widget.down_from(id, 0.0),
    }
}

fn right_position_item<W>(widget: W,
                          last_id: Option<widget::Id>,
                          parent_id: widget::Id,
                          first_item_margin: Scalar)
                          -> W
    where W: Widget
{
    match last_id {
        None => {
            widget.mid_left_with_margin_on(parent_id, first_item_margin).align_top_of(parent_id)
        }
        Some(id) => widget.right_from(id, 0.0),
    }
}
pub struct Items {
    item_indices: std::ops::Range<usize>,
    next_item_indices_index: usize,
    list_id: widget::Id,
    last_id: Option<widget::Id>,
    total_w: f64,
    first_left_id: Option<widget::Id>,
    acc_w: f64,
}

impl Items {
    /// Yield the next `Item` in the list.
    pub fn next<T>(&mut self, state: &State<T>, _ui: &Ui) -> Option<Item>
        where T: Clone + Send + 'static + Debug
    {

        let Items { ref mut item_indices,
                    ref mut next_item_indices_index,
                    ref mut last_id,
                    ref mut total_w,
                    ref mut acc_w,
                    ref mut first_left_id,
                    list_id } = *self;


        // Retrieve the `node_index` that was generated for the next `Item`.
        let node_index = match state.ids
                  .items
                  .get(*next_item_indices_index)
                  .map(|&id| id) {
            Some(node_index) => {
                *next_item_indices_index += 1;
                Some(node_index)
            }
            None => return None,
        };

        match (item_indices.next(), node_index) {
            (Some(i), Some(node_index)) => {
                let item = Item {
                    i: i,
                    last_id: *last_id,
                    widget_id: node_index,
                    parent_id: list_id,
                    first_left_id: first_left_id,
                    total_w: *total_w,
                    acc_w: acc_w,
                };
                *last_id = Some(node_index);
                Some(item)
            }
            _ => None,
        }
    }
}

impl<'a, T, W> DragDropList<'a, T, W>
    where T: Clone + Send + 'a + Debug,
          W: Widget + Draggable
{
    /// Create a button context to be built upon.
    pub fn new(values: &'a mut Vec<T>, widget_closure: Box<Fn(T) -> W>, item_width: f64) -> Self {
        DragDropList {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            values: values,
            widget_closure: widget_closure,
            item_width: item_width,
        }
    }
    builder_methods!{
        pub exit_id { style.exit_id = Option<Option<widget::Id>> }
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
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when an element exited, otherwise `None`.
    type Event = Option<T>;

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
        let item_idx_range = 0..self.values.len();
        if state.ids.items.len() < self.values.len() {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.items.resize(self.values.len(), id_gen));
        }
        let value_c = self.values.clone();
        widget::Rectangle::fill([w, h])
            .middle_of(id)
            .graphics_for(id)
            .color(style.color(&ui.theme))
            .set(state.ids.rect, ui);
        if state.temp.len() < value_c.len() {
            for _i in state.temp.len()..value_c.len() {
                if let Some(_v) = value_c.get(_i) {
                    state.update(|state| state.temp.push((None, _v.clone())));
                }
            }
        }
        let mut items = Items {
            list_id: id,
            item_indices: item_idx_range.clone(),
            next_item_indices_index: 0,
            last_id: None,
            first_left_id: None,
            total_w: w,
            acc_w: 0.0,
        };
        let mut c = 0;

        let mut values_c_iter = value_c.iter();
        if let Some(_) = state.last_release {
            while let (Some(item), Some(k_h)) =
                (items.next::<T>(&state, ui), values_c_iter.next()) {
                let widget = (*self.widget_closure)(k_h.clone());
                let k = item.set(widget, self.item_width, ui);
                state.update(|state| {
                                 if let Some(a) = state.temp.get_mut(c) {
                                     *a = (Some(k), k_h.clone());
                                 }
                                 state.last_release = None;
                             });

                c += 1;
            }

        } else {
            while let (Some(item), Some(k_h)) =
                (items.next::<T>(&state, ui), values_c_iter.next()) {
                let widget = (*self.widget_closure)(k_h.clone());
                item.set(widget.draggable(true), self.item_width, ui);
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
                if let Some(Some(exit_rect)) = style.exit_id {
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
        exit_id
    }
}
impl<'a, T, W> Colorable for DragDropList<'a, T, W>
    where T: Clone + Send + 'a + 'static + Debug,
          W: Widget + Draggable
{
    fn color(mut self, color: conrod_core::Color) -> Self {
        self.style.color = Some(color);
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
