use conrod_core::{widget, Positionable, Widget, Sizeable, color, Rect, Scalar, Color, Colorable,
             FontSize, Borderable};
use std;
pub use custom_widget::image_hover::{Hoverable, ImageHover};
use custom_widget::image_panels::{list_select, Panelable};
use custom_widget::bordered_image::Bordered;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemHistory<'a, P, A>
    where P: Panelable + 'a,
          A: Hoverable + Clone
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub panel_info: &'a mut P,
    pub overlay_blowup: &'a mut Option<usize>,
    pub corner_arrow: Option<A>,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default="(color::BLUE,[200.0,30.0,2.0])")]
    pub item_rect: Option<(Color, [f64; 3])>, //w,h, pad bottom
    #[conrod(default="[20.0,20.0,10.0,10.0]")]
    pub display_pic: Option<[f64; 4]>, // w,h,l,t
    #[conrod(default="[100.0,100.0,22.0,5.0]")]
    pub x_item_list: Option<[f64; 4]>, //w,h,l,t
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
    /// arrow size
    #[conrod(default = "40.0")]
    pub arrow_size: Option<f64>,
}

widget_ids! {
    struct Ids {
        display_pic,
        text,
        image_panel,
        corner_arrows[],
        rect,
        scrollbar
    }
}

/// Represents the unique, cached state for our ItemHistory widget.
pub struct State {
    ids: Ids,
}

impl<'a, P, A> ItemHistory<'a, P, A>
    where P: Panelable + 'a,
          A: Hoverable + Clone
{
    /// Create a button context to be built upon.
    pub fn new(panel_info: &'a mut P, overlay_blowup: &'a mut Option<usize>) -> Self {
        ItemHistory {
            panel_info: panel_info,
            corner_arrow: None,
            overlay_blowup: overlay_blowup,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
    pub fn corner_arrow(mut self, _h: A) -> Self {
        self.corner_arrow = Some(_h);
        self
    }
    builder_methods!{
        pub item_rect { style.item_rect = Some((Color,[f64;3])) }
        pub display_pic { style.display_pic = Some([f64;4]) }
        pub x_item_list { style.x_item_list = Some([f64;4]) }
        pub border { style.border = Some(Scalar) }
        pub border_color { style.border_color = Some(Color) }
        pub label_color{style.label_color = Some(Color)}
        pub label_font_size { style.label_font_size = Some(FontSize) }
        pub arrow_size {style.arrow_size=Some(f64)}
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, P, A> Widget for ItemHistory<'a, P, A>
    where P: Panelable + 'a,
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
        let widget::UpdateArgs { id, state, ui, rect, style, .. } = args;
        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let arrow_size = self.style.arrow_size(&ui.theme());
        if let Some((k, _rect, _)) = self.panel_info.display_pic() {
            let j = if let Some(_rect) = _rect {
                widget::Image::new(k).source_rectangle(Rect::from_corners(_rect.0, _rect.1))
            } else {
                widget::Image::new(k)
            };
            j.top_left_with_margins_on(id,
                                          style.display_pic(&ui.theme)[3],
                                          style.display_pic(&ui.theme)[2])
                .w_h(style.display_pic(&ui.theme)[0],
                     style.display_pic(&ui.theme)[1])
                .set(state.ids.display_pic, ui);
        }
        let label_color = self.style.label_color(&ui.theme);
        if let Some(_z) = self.panel_info.text() {
            widget::Text::new(&_z)
                .top_left_with_margins_on(id,
                                          style.display_pic(&ui.theme)[3],
                                          style.display_pic(&ui.theme)[2] +
                                          style.display_pic(&ui.theme)[0])
                .w(140.0)
                .h(40.0)
                .color(label_color)
                .set(state.ids.text, ui);
        }

        let rect_w = rect.w() - 140.0 - style.display_pic(&ui.theme)[2] +
                     style.display_pic(&ui.theme)[0];
        widget::Rectangle::outline([rect_w, rect.h()])
            .right_from(state.ids.text, 0.0)
            .color(style.item_rect(&ui.theme).0)
            .scroll_kids_vertically()
            .set(state.ids.rect, ui);
        let item_h = style.x_item_list(&ui.theme)[0];
        let (mut events, scrollbar) = list_select::ListSelect::multiple(self.panel_info.len())
            .flow_right()
            .item_size(item_h)
            .scrollbar_next_to()
            .w_h(700.0, style.x_item_list(&ui.theme)[1])
            .top_left_with_margins_on(state.ids.rect,
                                      style.x_item_list(&ui.theme)[3],
                                      style.x_item_list(&ui.theme)[2])
            .set(state.ids.image_panel, ui);
        let mut selected_widget_id_arr: Vec<(usize, widget::Id)> = vec![];
        while let Some(event) = events.next(ui, |i| self.panel_info.list_selected().contains(&i)) {
            use self::list_select::Event;
            match event {
                // For the `Item` events we instantiate the `List`'s items.
                Event::Item(item) => {
                    let mut j = match self.panel_info.list_selected().contains(&item.i) {
                        true => {
                            selected_widget_id_arr.push((self.panel_info.card_index(item.i),
                                                         item.widget_id));
                            self.panel_info.apply_closure(item.i).bordered()
                        }
                        false => self.panel_info.apply_closure(item.i),
                    };

                    j = j.border(style.border(&ui.theme))
                        .border_color(style.border_color(&ui.theme))
                        .w_h(style.x_item_list(&ui.theme)[0],
                             style.x_item_list(&ui.theme)[1]);

                    item.set(j, ui);
                }

                // The selection has changed.
                Event::Selection(selection) => {
                    selection.update_index_set(self.panel_info.list_selected_mut());
                    println!("selection {:?}", selection);
                }

                // The remaining events indicate interactions with the `ListSelect` widget.
                event => println!("{:?}", &event),
            }
        }
        if state.ids.corner_arrows.len() < selected_widget_id_arr.len() {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| {
                             state.ids.corner_arrows.resize(selected_widget_id_arr.len(), id_gen)
                         });
        }
        let mut selected_widget_id_iter = selected_widget_id_arr.iter();
        let mut state_corner_arrow_iter = state.ids.corner_arrows.iter();
        if let Some(_a) = self.corner_arrow {
            while let (Some(&(_ci, _si)), Some(_statec)) =
                (selected_widget_id_iter.next(), state_corner_arrow_iter.next()) {
                let j = ImageHover::new(_a.clone())
                    .w_h(arrow_size, arrow_size)
                    .top_right_with_margin_on(_statec.clone(), -2.0)
                    .set(_si, ui);
                for _c in j {
                    *self.overlay_blowup = Some(_ci);
                }
            }
        }
        if let Some(s) = scrollbar {
            s.set(ui);
        }
        Some(())
    }
}
