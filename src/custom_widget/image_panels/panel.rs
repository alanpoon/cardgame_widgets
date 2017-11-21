use conrod::{self, widget, Colorable, Positionable, Widget, Sizeable, color};
use custom_widget::image_panels::item_history;
use std;
use std::collections::HashSet;
pub trait Panelable<'a> {
    fn text(&self) -> Option<String>;
    fn display_pic(&self) -> Option<(image::Id, ([f64; 2], [f64; 2]))>;
    fn list_image(&self) -> Vec<image::Id, ([f64; 2], [f64; 2])>;
    fn list_selected(&self) -> &'a mut HashSet<usize, (image::Id, ([f64; 2], [f64; 2]))>;
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ImagePanels<'a, P>
    where P: Panelable + 'a
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub panel_infos: Vec<P>,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Border the row of images
    #[conrod(default="(color::BLUE,[200.0,30.0,2.0])")]
    pub item_rect: Option<(conrod::Color, [f64; 3])>, //w,h, pad bottom
    /// Weight, height of the display pic, left top from corner
    #[conrod(default="[20.0,20.0,10.0,10.0]")]
    pub display_pic: Option<[f64; 4]>, // w,h,l,t
    /// Weight, height of the display pic, left top from corner
    #[conrod(default="[100.0,50.0,22.0,5.0]")]
    pub x_item_list: Option<[f64; 4]>, //w,h,l,t
    #[conrod(default="260.0")]
    pub y_item_height: Option<f64>,
    /// Width of the border surrounding the Image List Item
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border surrounding the Image List Item
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
}

widget_ids! {
    struct Ids {
        panel
    }
}

/// Represents the unique, cached state for our ImagePanels widget.
pub struct State {
    ids: Ids,
}

impl<'a, P> ImagePanels<'a, P>
    where P: Panelable + 'a
{
    /// Create a button context to be built upon.
    pub fn new(panel_infos: P) -> Self {
        ImagePanels {
            panel_infos: panel_infos,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    builder_methods!{
        pub item_rect { style.item_rect = Some((conrod::Color,[f64;3])) }
        pub display_pic { style.display_pic = Some([f64;4]) }
        pub x_item_list { style.x_item_list = Some([f64;4]) }
        pub y_item_height {style.y_item_height = Some(f64)}
        pub border { style.border = Some(Scalar) }
        pub border_color { style.border_color = Some(Color) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, P> Widget for ImagePanels<'a, P>
    where P: Panelable + 'a
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
    fn update(self, args: widget::UpdateArgs<Self>) -> std::option::Option<()> {
        let widget::UpdateArgs { id, state, ui, rect, style, .. } = args;
        let y_item_height = style.y_item_height(&ui.theme);
        let (mut items, scrollbar) = widget::List::flow_down(self.panel_infos.len())
            .item_size(y_item_height)
            .scrollbar_on_top()
            .middle_of(ids)
            .wh_of(ids)
            .set(ids.panel, ui);
        let mut panel_iter = self.panel_infos.iter();
        while let (Some(item), Some(_panel)) = (items.next(ui), panel_iter.next()) {
            let i = item.i;
            item_history::ItemHistory::new(_panel)
        }

        if let Some(s) = scrollbar {
            s.set(ui)
        }
        Some(())
    }
}
