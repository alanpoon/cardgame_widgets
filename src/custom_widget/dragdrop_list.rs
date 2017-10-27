use conrod::{self, widget, Positionable, Widget, color, Ui, UiCell, graph, Sizeable, Dimensions,
             Theme, Rect};
use conrod::position::Point;
use std;
use conrod::position::Scalar;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct DragDropList {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// See the Style struct below.
    style: Style,
    num: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {}

widget_ids! {
    struct Ids {
      items[],
      drag_items[],
      drag_canvas
    }
}

/// Represents the unique, cached state for our DragDropList widget.
pub struct State {
    ids: Ids,
    rects: Vec<[f64; 2]>,
    acc_w: f64,
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
    pub fn set<W>(mut self, widget: W, width: Scalar, ui: &mut UiCell) -> widget::Id
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
                     
                     //.parent(self.parent_id)
                .set(widget_id, ui);


        } else {
            *(self.acc_w) = acc_w_c + width;
            if let None = last_id {
                *(self.first_left_id) = Some(widget_id);
            }
            widget.and(|w| right_position_item(w, last_id, parent_id, 0.0))
                     //.parent(self.parent_id)
                     .set(widget_id, ui);
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
    pub fn next(&mut self, ui: &Ui) -> Option<Item> {

        let Items { ref mut item_indices,
                    ref mut next_item_indices_index,
                    ref mut last_id,
                    ref mut total_w,
                    ref mut acc_w,
                    ref mut first_left_id,
                    list_id } = *self;


        // Retrieve the `node_index` that was generated for the next `Item`.
        let node_index =
            match ui.widget_graph()
                      .widget(list_id)
                      .and_then(|container| container.unique_widget_state::<DragDropList>())
                      .and_then(|&graph::UniqueWidgetState { ref state, .. }| {
                                    state.ids
                                        .items
                                        .get(*next_item_indices_index)
                                        .map(|&id| id)
                                }) {
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
pub struct DragItem {
    pub i: usize,
    pub widget_id: widget::Id,
    pub last_id: Option<widget::Id>,
    pub parent_id: widget::Id,
}
impl DragItem {
    pub fn set<W>(mut self, widget: W, xy: Point, ui: &mut UiCell) -> W::Event
        where W: Widget
    {
        widget.xy_relative_to(self.parent_id, xy).graphics_for(self.parent_id).set(self.widget_id,
                                                                                   ui)
    }
}
pub struct DragItems {
    item_indices: std::ops::Range<usize>,
    next_item_indices_index: usize,
    list_id: widget::Id,
    last_id: Option<widget::Id>,
}
impl DragItems {
    pub fn next(&mut self, ui: &Ui) -> Option<DragItem> {

        let DragItems { ref mut item_indices,
                        ref mut next_item_indices_index,
                        ref mut last_id,
                        list_id } = *self;


        // Retrieve the `node_index` that was generated for the next `Item`.
        let node_index =
            match ui.widget_graph()
                      .widget(list_id)
                      .and_then(|container| container.unique_widget_state::<DragDropList>())
                      .and_then(|&graph::UniqueWidgetState { ref state, .. }| {
                                    state.ids
                                        .drag_items
                                        .get(*next_item_indices_index)
                                        .map(|&id| id)
                                }) {
                Some(node_index) => {
                    *next_item_indices_index += 1;
                    Some(node_index)
                }
                None => return None,
            };

        match (item_indices.next(), node_index) {
            (Some(i), Some(node_index)) => {
                let item = DragItem {
                    i: i,
                    last_id: *last_id,
                    widget_id: node_index,
                    parent_id: list_id,
                };
                *last_id = Some(node_index);
                Some(item)
            }
            _ => None,
        }
    }
}
impl DragDropList {
    /// Create a button context to be built upon.
    pub fn new(num: usize) -> Self {
        DragDropList {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            num: num,
        }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl Widget for DragDropList {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = (Items, DragItems);

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            rects: vec![],
            acc_w: 0.0,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, mut ui, .. } = args;
        let w = rect.w();
        let item_idx_range = 0..self.num;
        if state.ids.items.len() < self.num {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.items.resize(self.num, id_gen));
        }
        if state.ids.drag_items.len() < self.num {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.drag_items.resize(self.num, id_gen));
        }
        let items = Items {
            list_id: id,
            item_indices: item_idx_range.clone(),
            next_item_indices_index: 0,
            last_id: None,
            first_left_id: None,
            total_w: w,
            acc_w: 0.0,
        };
        let dragitems = DragItems {
            list_id: id,
            next_item_indices_index: 0,
            item_indices: item_idx_range,
            last_id: None,
        };
        (items, dragitems)
    }
}
