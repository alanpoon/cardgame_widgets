use conrod::{self, widget, Colorable, Positionable, Widget, Sizeable, color};
use std;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemHistory<'a,T> where T:Clone {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub selected: &'a mut Option<(usize,usize)>,
    pub y:usize,
    pub x_images:&'a Vec<T>,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default="(color::BLUE,[200.0,30.0,2.0])")]
    pub item_rect: Option<(conrod::Color, [f64; 3])>, //w,h, pad bottom
    #[conrod(default="[20.0,20.0,10.0,10.0]")]
    pub item_image: Option<[f64; 4]>, // w,h,l,t
    #[conrod(default="(theme.label_color,theme.font_id,theme.font_size_medium,[100.0,50.0,22.0,5.0])")]
    pub item_text: Option<(conrod::Cx_imagesolor,
                               Option<conrod::text::font::Id>,
                               conrod::FontSize,
                               [f64; 4])>, //RGB,w,h,l,t
}

widget_ids! {
    struct Ids {
        display_pic,
        name,
        image_panel,
        rect,
        scrollbar
    }
}

/// Represents the unique, cached state for our ItemHistory widget.
pub struct State {
    ids: Ids,
}

impl<'a,T> ItemHistory<'a,T> where T:Clone {
    /// Create a button context to be built upon.
    pub fn new(selected: &'a mut Option<(usize,usize)>,y:usize,x_images:&'a Vec<T>) -> Self {
        ItemHistory {
            selected: selected,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            y:usize,
            x_images:x_images
        }
    }


    builder_methods!{
        pub item_rect { style.item_rect = Some((conrod::Color,[f64;3])) }
        pub item_image { style.item_image = Some([f64;4]) }
        pub item_text { style.item_text = Some((conrod::Color,Option<conrod::text::font::Id>,conrod::FontSize,[f64;4])) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for ItemHistory<'a> {
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
        let mut z = self.message.name.clone();
        z.push_str(": ");
        if let Some(k) = self.message.image_id {
            widget::Image::new(k)
                .top_left_with_margins_on(id,
                                          style.item_image(&ui.theme)[3],
                                          style.item_image(&ui.theme)[2])
                .w_h(style.item_image(&ui.theme)[0],
                     style.item_image(&ui.theme)[1])
                .set(state.ids.display_pic, ui);
        }
        widget::Text::new(&z)
            .top_left_with_margins_on(id,
                                      style.item_image(&ui.theme)[3],
                                      style.item_image(&ui.theme)[2] +
                                      style.item_image(&ui.theme)[0])
            .w(120.0)
            .set(state.ids.name, ui);
        let rect_w = rect.w() - 140.0 - style.item_image(&ui.theme)[2] +
                     style.item_image(&ui.theme)[0];
        widget::Rectangle::outline([rect_w, rect.h()])
            .right_from(state.ids.name, 0.0)
            .color(style.item_rect(&ui.theme).0)
            .scroll_kids_vertically()
            .set(state.ids.rect, ui);
       let item_h = 230.0;
       let (mut events, scrollbar) = widget::ListSelect::single(self.x_images.len())
                    .flow_down()
                    .item_size(item_h)
                    .scrollbar_next_to()
                    .w_h(700.0, 260.0)
                    .top_left_with_margins_on(state.ids.rect,
                                      style.item_text(&ui.theme).3[3],
                                      style.item_text(&ui.theme).3[2])
                    .set(ids.image_panel, ui);
        let y_c = self.y.clone();
         while let Some(event) = events.next(ui, |i| {
             match (&self.y,&self.selected){
                (ref _this_y,&Some((ref _x,ref _y)))=>{
                    if (*_this_y == *_y) &(i==*_x){
                        true
                    } else{false}
                },
                _=>{
                    false
                }
             }
             }) {
                    use conrod::widget::list_select::Event;
                    match event {
                        // For the `Item` events we instantiate the `List`'s items.
                        Event::Item(item) => {
                            let card_index = inked.get(item.i).unwrap();
                            let color=if let Some(_ci) = card_index_sel{
                                if _ci ==inked.get(item.i).unwrap(){
                                    conrod::color::YELLOW
                                } else {
                                    conrod::color::LIGHT_GREY
                                }
                            } else{conrod::color::LIGHT_GREY
                            };
                            let (_image_id, _rect, _) = in_game::get_card_widget_image_portrait(card_index.clone(), card_images, appdata);
                            let button = widget::Button::image(_image_id)
                                .source_rectangle(_rect)
                                .color(color);
                            item.set(button, ui);
                        },

                        // The selection has changed.
                        Event::Selection(idx) => {
                          card_index_sel = Some(inked.get(idx).unwrap())
                        },

                        // The remaining events indicate interactions with the `ListSelect` widget.
                        event => println!("{:?}", &event),
                    }
                }  
        widget::Scrollbar::y_axis(state.ids.rect).auto_hide(false).set(state.ids.scrollbar, ui);

        Some(())
    }
}
