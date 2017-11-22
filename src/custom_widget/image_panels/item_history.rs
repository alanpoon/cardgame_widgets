use conrod::{widget, Positionable, Widget, Sizeable, color, Rect, Scalar, Color, Colorable};
use std;
use custom_widget::bordered_image::BorderedImage;
use custom_widget::image_panels::Panelable;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemHistory<'a, P>
    where P: Panelable + 'a
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub panel_info: &'a mut P,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default="(color::BLUE,[200.0,30.0,2.0])")]
    pub item_rect: Option<(Color, [f64; 3])>, //w,h, pad bottom
    #[conrod(default="[20.0,20.0,10.0,10.0]")]
    pub display_pic: Option<[f64; 4]>, // w,h,l,t
    #[conrod(default="[100.0,50.0,22.0,5.0]")]
    pub x_item_list: Option<[f64; 4]>, //w,h,l,t
    /// Width of the border surrounding the Image List Item
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border surrounding the Image List Item
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
}

widget_ids! {
    struct Ids {
        display_pic,
        text,
        image_panel,
        rect,
        scrollbar
    }
}

/// Represents the unique, cached state for our ItemHistory widget.
pub struct State {
    ids: Ids,
}

impl<'a, P> ItemHistory<'a, P>
    where P: Panelable + 'a
{
    /// Create a button context to be built upon.
    pub fn new(panel_info: &'a mut P) -> Self {
        ItemHistory {
            panel_info: panel_info,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    builder_methods!{
        pub item_rect { style.item_rect = Some((Color,[f64;3])) }
        pub display_pic { style.display_pic = Some([f64;4]) }
        pub x_item_list { style.x_item_list = Some([f64;4]) }
        pub border { style.border = Some(Scalar) }
        pub border_color { style.border_color = Some(Color) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, P> Widget for ItemHistory<'a, P>
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
        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //

        if let Some((k, _rect)) = self.panel_info.display_pic() {
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
        if let Some(_z) = self.panel_info.text() {
            widget::Text::new(&_z)
                .top_left_with_margins_on(id,
                                          style.display_pic(&ui.theme)[3],
                                          style.display_pic(&ui.theme)[2] +
                                          style.display_pic(&ui.theme)[0])
                .w(120.0)
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
        let list_image = self.panel_info.list_image().clone();
        let (mut events, scrollbar) = widget::ListSelect::multiple(list_image.len())
            .flow_right()
            .item_size(item_h)
            .scrollbar_next_to()
            .w_h(700.0, style.x_item_list(&ui.theme)[1])
            .top_left_with_margins_on(state.ids.rect,
                                      style.x_item_list(&ui.theme)[3],
                                      style.x_item_list(&ui.theme)[2])
            .set(state.ids.image_panel, ui);
        let list_selected = self.panel_info.list_selected().clone();
        let list_image_c = self.panel_info.list_image().clone();
        while let Some(event) = events.next(ui, |i| list_selected.contains(&i)) {
            use conrod::widget::list_select::Event;
            match event {
                // For the `Item` events we instantiate the `List`'s items.
                Event::Item(item) => {
                    let &(ref _image_id, ref _rect) = list_image_c.get(item.i).unwrap();
                    let _rect_c = _rect.clone();
                    let mut j = match self.panel_info.list_selected().contains(&item.i) {
                        true => BorderedImage::new(_image_id.clone()).bordered(),
                        false => BorderedImage::new(_image_id.clone()),
                    };
                    j = j.source_rectangle(Rect::from_corners(_rect_c.0, _rect_c.1))
                        .border(style.border(&ui.theme))
                        .border_color(style.border_color(&ui.theme))
                        .w_h(style.x_item_list(&ui.theme)[0],
                             style.x_item_list(&ui.theme)[1]);
                    item.set(j, ui);
                }

                // The selection has changed.
                Event::Selection(selection) => {
                    selection.update_index_set(&mut self.panel_info.list_selected());
                }

                // The remaining events indicate interactions with the `ListSelect` widget.
                event => println!("{:?}", &event),
            }
        }
        if let Some(s) = scrollbar {
            s.set(ui);
        }

        Some(())
    }
}
