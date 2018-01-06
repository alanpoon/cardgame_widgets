use conrod::{widget, Color, Colorable, Positionable, UiCell, Widget, Sizeable, Rect, text};
use custom_widget::player_info::item::{Icon, IconStruct};
use std::iter::once;
use text::get_font_size_wh;
//Player_info list all player's item, at the end, there is some arrow animation that opens another overlay

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct List<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub icon_vec: Vec<IconStruct>,
    /// See the Style struct below.
    style: Style,
    pub overlay: &'a mut bool,
    pub maybe_label: Option<&'a str>,
}

#[derive(Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// The color of the AnimatedButton's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The ID of the font used to display the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<text::font::Id>>,
}

widget_ids! {
    struct Ids {
        rect,
        icon_vec,
        arrow1,
        arrow2,
        arrow3,
        player_info,

    }
}

/// Represents the unique, cached state for our widget.
pub struct State {
    ids: Ids,
    frame: u16,
    selected: Option<usize>,
    selected_id: Option<widget::Id>,
    selected_xy: Option<[f64; 2]>,
}

impl<'a> List<'a> {
    /// Create a button context to be built upon.
    pub fn new(icon_vec: Vec<IconStruct>, overlay: &'a mut bool) -> Self {
        List {
            icon_vec: icon_vec,
            common: widget::CommonBuilder::default(),
            overlay: overlay,
            style: Style::default(),
            maybe_label: None,
        }
    }
    builder_methods!{
        pub label_color { style.label_color = Some(Color) }
        pub label { maybe_label = Some(&'a str) }
    }
    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for List<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = (Option<usize>, Option<widget::Id>, Option<[f64; 2]>);

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            frame: 0,
            selected: None,
            selected_id: None,
            selected_xy: None,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        let (interaction, _times_triggered) = interaction_and_times_triggered(id, ui);
        let _dim = rect.dim();
        let default_color = self.style.color(&ui.theme);
        let rect_c = match interaction {
            Interaction::Idle => default_color,
            Interaction::Hover => default_color.highlighted(),
            Interaction::Press => default_color.highlighted(),
        };

        rectangle_fill(id, state.ids.rect, rect, rect_c, ui);
        //  let font_id = self.style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
        if let Some(_a) = self.maybe_label {
            let fontsize = get_font_size_wh(_dim[0] * 0.5, _dim[1], _a);
            widget::Text::new(_a)
                .top_left_with_margins_on(id, _dim[1] * 0.1, 0.0)
                .font_size(fontsize)
               // .and_then(font_id, widget::Text::font_id)
                .color(default_color.plain_contrast())
                .w(_dim[0] * 0.3)
                .h_of(id)
                .set(state.ids.player_info, ui);
        }

        let item_size = _dim[0] * 0.5 / self.icon_vec.len() as f64;
        let (mut events, _scrollbar) = widget::ListSelect::single(self.icon_vec.len())
            .flow_right()
            .top_left_with_margins_on(id, 0.0, _dim[0] * 0.3)
            .item_size(item_size)
            .w(_dim[0] * 0.6)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.icon_vec, ui);
        while let Some(event) = events.next(ui, |i| {
            let mut y = false;
            if let Some(_s) = state.selected {
                if i == _s {
                    y = true;
                }
            }
            y
        }) {
            use conrod::widget::list_select::Event;
            match event {
                Event::Item(item) => {
                    if let Some(ref info) = self.icon_vec.get(item.i) {
                        let mut j = Icon::new(info.clone().clone())
                            .label_color(self.style.label_color(&ui.theme));
                        if let Some(_s) = state.selected {
                            if _s == item.i {
                                j = j.bordered();
                                let xy = ui.xy_of(item.widget_id);
                                state.update(|state| state.selected_xy = xy);
                                state.update(|state| state.selected_id = Some(item.widget_id));
                            }
                        }
                        item.set(j, ui);
                    }
                }
                Event::Selection(id) => {
                    if let Some(_c) = state.selected {
                        if _c == id {
                            *self.overlay = false;
                            state.update(|state| state.selected = None);
                        } else {
                            *self.overlay = true;
                            state.update(|state| state.selected = Some(id));
                        }
                    } else {
                        *self.overlay = true;
                        state.update(|state| state.selected = Some(id));
                    }
                }
                _ => {}
            }

        }

        let (left, top, right) = if self.overlay.clone() {
            ([-_dim[0] * 0.2 / 3.0, -_dim[0] * 0.2 / 3.0],
             [0.0, 0.0],
             [-_dim[0] * 0.2 / 3.0, _dim[0] * 0.2 / 3.0])
        } else {
            ([0.0, -_dim[0] * 0.2 / 3.0], [-_dim[0] * 0.2 / 3.0, 0.0], [0.0, _dim[0] * 0.2 / 3.0])
        };
        let (lefz, midz, rigz) = if self.overlay.clone() {
            (0.0, 1.0, 2.0)
        } else {
            (2.0, 1.0, 0.0)
        };
        let points = once(left).chain(once(top)).chain(once(right));
        if (state.frame as f64 / 60.0).floor() == lefz {
            widget::PointPath::centred(points.clone())
              //  .w(10.0)
               // .h(10.0)
                .align_middle_y_of(state.ids.icon_vec)
                .right_from(state.ids.icon_vec, 5.0)
                .set(state.ids.arrow1, ui);
        } else if (state.frame as f64 / 60.0).floor() == midz {
            widget::PointPath::centred(points.clone())
              //  .w(10.0)
              //  .h(10.0)
                .align_middle_y_of(state.ids.icon_vec)
                .right_from(state.ids.arrow1, 5.0)
                .set(state.ids.arrow2, ui);
        } else if (state.frame as f64 / 60.0).floor() == rigz {
            widget::PointPath::centred(points)
                //.w(20.0)
              //  .w(10.0)
              //  .h(10.0)
                .align_middle_y_of(state.ids.icon_vec)
                .right_from(state.ids.arrow2, 5.0)
                .set(state.ids.arrow3, ui);
        }

        state.update(|state| state.frame += 2);
        if state.frame > 180 {
            state.update(|state| state.frame = 0);
        }
        (state.selected, state.selected_id, state.selected_xy)
    }
}

fn rectangle_fill(button_id: widget::Id,
                  rectangle_id: widget::Id,
                  rect: Rect,
                  color: Color,
                  ui: &mut UiCell) {
    // BorderedRectangle widget.
    let dim = rect.dim();
    widget::Rectangle::fill_with(dim, color)
        .middle_of(button_id)
        .graphics_for(button_id)
        .set(rectangle_id, ui);
}
impl<'a> Colorable for List<'a> {
    fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }
}

#[derive(Copy, Clone,Debug)]
enum Interaction {
    Idle,
    Hover,
    Press,
}
fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {
    let input = ui.widget_input(button_id);
    let interaction = input.mouse().map_or(Interaction::Idle,
                                           |mouse| if mouse.buttons.left().is_down() {
                                               Interaction::Press
                                           } else {
                                               Interaction::Hover
                                           });
    let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
    (interaction, times_triggered)
}
