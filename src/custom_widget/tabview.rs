use conrod_core::{self, widget, Colorable, Positionable, Widget, Sizeable, color, Borderable, Ui, UiCell,
             graph, Scalar};
use std;
const MARGIN: conrod_core::Scalar = 5.0;
#[derive(Debug)]

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct TabView<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    pub tab_names: Vec<&'a str>,
    enabled: bool,
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
    #[conrod(default = "90.0")]
    pub bar_thickness: Option<Scalar>,
}

widget_ids! {
 pub struct Ids {
        master,
        items[],
        labels[],
        tabs,
    }
}

/// Represents the unique, cached state for our TabView widget.
pub struct State {
    pub ids: Ids,
}
/// The data necessary for instantiating a single item within a `List`.
#[derive(Copy, Clone, Debug)]
pub struct Item {
    pub i: usize,
    /// The id generated for the widget.
    pub widget_id: widget::Id,
    pub last_id: Option<widget::Id>,
    pub parent_id: widget::Id,
}
impl Item {
    /// Sets the given widget as the widget to use for the item.
    ///
    /// Sets the:
    /// - position of the widget.
    /// - dimensions of the widget.
    /// - parent of the widget.
    /// - and finally sets the widget within the `Ui`.
    pub fn set<W>(self, widget: W, ui: &mut UiCell) -> W::Event
        where W: Widget
    {
        let Item { widget_id, parent_id, .. } = self;

        widget.middle_of(parent_id).padded_wh_of(parent_id, MARGIN).set(widget_id, ui)
    }
}
/// An `Iterator` yielding each `Item` in the list.
pub struct Items {
    item_indices: std::ops::Range<usize>,
    next_item_indices_index: usize,
    next_item_indices_index2: usize,
    list_id: widget::Id,
    last_id: Option<widget::Id>,
}
impl Items {
    /// Yield the next `Item` in the list.
    pub fn next(&mut self, ui: &Ui) -> Option<Item> {
        let Items { ref mut item_indices,
                    ref mut next_item_indices_index,
                    ref mut next_item_indices_index2,
                    ref mut last_id,
                    list_id } = *self;

        // Retrieve the `node_index` that was generated for the next `Item`.
        let node_index =
            match ui.widget_graph()
                      .widget(list_id)
                      .and_then(|container| container.unique_widget_state::<TabView>())
                      .and_then(|&graph::UniqueWidgetState { ref state, .. }| {
                                    state.ids
                                        .labels
                                        .get(*next_item_indices_index)
                                        .map(|&id| id)
                                }) {
                Some(node_index) => {
                    *next_item_indices_index += 1;
                    Some(node_index)
                }
                None => return None,
            };
        let parent_node =
            match ui.widget_graph()
                      .widget(list_id)
                      .and_then(|container| container.unique_widget_state::<TabView>())
                      .and_then(|&graph::UniqueWidgetState { ref state, .. }| {
                                    state.ids
                                        .items
                                        .get(*next_item_indices_index2)
                                        .map(|&id| id)
                                }) {
                Some(node_index) => {
                    *next_item_indices_index2 += 1;
                    Some(node_index)
                }
                None => return None,
            };


        match (item_indices.next(), node_index, parent_node) {
            (Some(i), Some(node_index), Some(parent_node)) => {
                let item = Item {
                    i: i,
                    last_id: *last_id,
                    widget_id: node_index,
                    parent_id: parent_node,
                };
                *last_id = Some(node_index);
                Some(item)
            }
            _ => None,
        }
    }
}

impl<'a> TabView<'a> {
    /// Create a button context to be built upon.
    pub fn new(tab_names: Vec<&'a str>) -> Self {
        TabView {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            enabled: true,
            tab_names: tab_names,
        }
    }
    builder_methods!{
        pub bar_thickness { style.bar_thickness = Some(f64) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for TabView<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<Items>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Option<(Items)> {
        let widget::UpdateArgs { rect, id, state, ui, .. } = args;
        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //

        // Construct our main `Tab` tree.
        let (_, _, w, h) = rect.x_y_w_h();
        let num = self.tab_names.len();
        let item_idx_range = 0..num;
        if state.ids.items.len() < num {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.items.resize(num, id_gen));
        }
        if state.ids.labels.len() < num {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.labels.resize(num, id_gen));
        }
        let k: Vec<(widget::Id, &str)> = state.ids
            .items
            .iter()
            .zip(self.tab_names.iter())
            .map(|(&z, &k)| (z, k))
            .collect();
        // Here we make some canvas `Tabs` in the middle column.
        widget::Tabs::new(&k)
            .color(color::BLUE)
            .label_color(color::WHITE)
            .w_h(w, h)
            .middle()
            .bar_thickness(self.style.bar_thickness(&ui.theme))
            .border_color(color::LIGHT_CHARCOAL)
            .set(state.ids.tabs, ui);

        let items = Items {
            list_id: id,
            item_indices: item_idx_range,
            next_item_indices_index: 0,
            next_item_indices_index2: 0,
            last_id: None,
        };
        Some(items)
    }
}
