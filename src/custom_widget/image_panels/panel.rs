use conrod_core::{self, widget, Positionable, Widget, Sizeable, color, Scalar, Color, FontSize};
use custom_widget::image_panels::{item_history, Panelable};
use custom_widget::image_hover::Hoverable;
use std;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ImagePanels<'b, P, A>
    where P: Panelable + 'b,
          A: Hoverable + Clone
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub panel_infos: &'b mut Vec<P>,
    pub overlay_blowup: &'b mut Option<usize>,
    pub corner_arrow: Option<A>,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Border the row of images
    #[conrod(default="(color::BLUE,[200.0,30.0,2.0])")]
    pub item_rect: Option<(conrod_core::Color, [f64; 3])>, //w,h, pad bottom
    /// Weight, height of the display pic, left top from corner
    #[conrod(default="[20.0,20.0,10.0,10.0]")]
    pub display_pic: Option<[f64; 4]>, // w,h,l,t
    /// Weight, height of the display pic, left top from corner
    #[conrod(default="[100.0,100.0,22.0,5.0]")]
    pub x_item_list: Option<[f64; 4]>, //w,h,l,t
    #[conrod(default="190.0")]
    pub y_item_height: Option<f64>,
    /// Width of the border surrounding the Image List Item
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border surrounding the Image List Item
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// The LabelColor
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The font size of the Button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<FontSize>,
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

impl<'b, P, A> ImagePanels<'b, P, A>
    where P: Panelable + 'b,
          A: Hoverable + Clone
{
    /// Create a button context to be built upon.
    pub fn new(panel_infos: &'b mut Vec<P>, overlay_blowup: &'b mut Option<usize>) -> Self {
        ImagePanels {
            panel_infos: panel_infos,
            overlay_blowup: overlay_blowup,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            corner_arrow: None,
        }
    }
    pub fn corner_arrow(mut self, _h: A) -> Self {
        self.corner_arrow = Some(_h);
        self
    }
    builder_methods!{
        pub item_rect { style.item_rect = Some((conrod_core::Color,[f64;3])) }
        pub display_pic { style.display_pic = Some([f64;4]) }
        pub x_item_list { style.x_item_list = Some([f64;4]) }
        pub y_item_height {style.y_item_height = Some(f64)}
        pub border { style.border = Some(Scalar) }
        pub border_color { style.border_color = Some(Color) }
        pub label_color{style.label_color = Some(Color)}
        pub label_font_size { style.label_font_size = Some(FontSize) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'b, P, A> Widget for ImagePanels<'b, P, A>
    where P: Panelable + 'b,
          A: Hoverable + Clone
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
        let widget::UpdateArgs { id, state, ui, style, .. } = args;
        let y_item_height = style.y_item_height(&ui.theme);
        let (mut items, scrollbar) = widget::List::flow_down(self.panel_infos.len())
            .item_size(y_item_height)
            .scrollbar_on_top()
            .middle_of(id)
            .wh_of(id)
            .set(state.ids.panel, ui);
        let mut panel_iter = self.panel_infos.iter_mut();
        if let Some(_corner_arrow) = self.corner_arrow {
            while let (Some(item), Some(_panel)) = (items.next(ui), panel_iter.next()) {
                //let i = item.i;
                let mut j = item_history::ItemHistory::new(_panel, self.overlay_blowup)
                    .label_color(self.style.label_color(&ui.theme));

                j = j.corner_arrow(_corner_arrow.clone());

                item.set(j, ui);
            }
        }

        if let Some(s) = scrollbar {
            s.set(ui)
        }
        Some(())
    }
}
