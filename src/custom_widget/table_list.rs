use conrod::{self, widget, Positionable, Widget, Sizeable, Labelable, Colorable};
use custom_widget::pad_text_button;

pub trait TableListTexts {
    fn text_ready(&self) -> &'static str;
    fn text_leave(&self) -> &'static str;
    fn text_join(&self) -> &'static str;
    fn text_playergame(&self) -> &'static str;
    fn text_changeto(&self) -> &'static str;
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct TableList<'a, T: TableListTexts + 'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub ready: Box<Fn() + 'a>,
    pub join: Box<Fn() + 'a>,
    pub leave: Box<Fn() + 'a>,
    pub change_table_space_closure: Box<Fn(usize) + 'a>,
    pub players: &'a Vec<String>, //width%,text
    pub appdata: &'a T,
    pub table_space: usize,
    pub max_space: usize,
    pub joined: bool,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
}

widget_ids! {
    struct Ids {
        rect,
        ready_join,
        leave,
        table_space,
        players_text,
        change_table_space[],
    }
}

/// Represents the unique, cached state for our TableList widget.
pub struct State {
    ids: Ids,
}

impl<'a, T> TableList<'a, T>
    where T: TableListTexts + 'a
{
    /// Create a button context to be built upon.
    pub fn new(appdata: &'a T,
               ready: Box<Fn() + 'a>,
               join: Box<Fn() + 'a>,
               leave: Box<Fn() + 'a>,
               change_table_space_closure: Box<Fn(usize) + 'a>,
               players: &'a Vec<String>,
               table_space: usize,
               max_space: usize,
               joined: bool)
               -> Self {
        TableList {
            appdata: appdata,
            ready: ready,
            join: join,
            leave: leave,
            change_table_space_closure: change_table_space_closure,
            players: players,
            table_space: table_space,
            max_space: max_space,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            joined: joined,
        }
    }

    builder_methods!{
        pub label_color { style.label_color = Some(conrod::Color) }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, T> Widget for TableList<'a, T>
    where T: TableListTexts + 'a
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
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, style, .. } = args;
        let (_x, _y, w, h) = rect.x_y_w_h();
        let pad = 0.01 * w;
        widget::Rectangle::outline([w, h]).top_left_of(id).set(state.ids.rect, ui);
        if self.joined {
            let r_but = widget::Button::new()
                .label(self.appdata.text_ready())
                .label_color(style.label_color((&ui.theme)))
                .w((w - (2.0 * pad)) / 8.0)
                .h(0.4 * h)
                .top_left_with_margins_on(id, 0.01 * h, 0.01 * w)
                .set(state.ids.ready_join, ui);
            for _ in r_but {
                (*self.ready)();
            }
            let l_but = widget::Button::new()
                .label(self.appdata.text_leave())
                .label_color(style.label_color((&ui.theme)))
                .w((w - (2.0 * pad)) / 8.0)
                .h(0.4 * h)
                .down_from(state.ids.ready_join, 2.0)
                .set(state.ids.leave, ui);
            for _ in l_but {
                (*self.leave)();
            }
        } else {
            let r_but = widget::Button::new()
                .label(self.appdata.text_join())
                .label_color(style.label_color((&ui.theme)))
                .w((w - (2.0 * pad)) / 8.0)
                .h(0.4 * h)
                .top_left_with_margins_on(id, 0.01 * h, 0.01 * w)
                .set(state.ids.ready_join, ui);
            for _ in r_but {
                (*self.join)();
            }
        }

        let change_table_space_but_w = w / 8.0;
        let f = format!("{} {}", self.table_space, self.appdata.text_playergame());
        widget::Text::new(&f)
            .w(change_table_space_but_w)
            .h(0.8 * h)
            .right_from(state.ids.ready_join, 2.0)
            .color(style.label_color((&ui.theme)))
            .set(state.ids.table_space, ui);
        let mut players_string = "".to_owned();
        let num = self.players.len();
        let play_c = self.players.clone();
        let mut a = 0;
        for b in play_c {
            players_string.push_str(&b);
            if a < num - 1 {
                let k = ",".to_owned();
                players_string.push_str(&k);
            }
            a += 1;
        }
        widget::Text::new(&players_string)
            .w((w - (2.0 * pad)) / 7.0)
        //    .padded_w_of(state.ids.table_space, 0.0)
        //    .mid_top_with_margin_on(button_id, 4.0)
            .h(0.8 * h)
            .right_from(state.ids.table_space, 2.0)
            .color(style.label_color((&ui.theme)))
            .set(state.ids.players_text, ui);

        if state.ids.change_table_space.len() < self.max_space {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.change_table_space.resize(self.max_space, id_gen));
        }

        let mut change_table_space_iter = state.ids
            .change_table_space
            .iter()
            .enumerate();
        if self.joined {
            let mut iplay = self.players.len();
            while (iplay) < self.table_space {
                if let Some((counter, &sym)) = change_table_space_iter.next() {
                    if iplay != 1 {
                        let f = format!("{} {} {}",
                                        self.appdata.text_changeto(),
                                        iplay,
                                        self.appdata.text_playergame());
                        let but = pad_text_button::Button::new(4)
                            .label(&f)
                            .label_color(style.label_color((&ui.theme)))
                            .right_from(state.ids.players_text,
                                        (counter as f64) * change_table_space_but_w)
                            .w_h(change_table_space_but_w, 0.8 * h)
                            .set(sym, ui);
                        for _i in but {
                            (*self.change_table_space_closure)(iplay);
                        }
                    } else {
                        widget::Rectangle::outline([0.0, 0.0]).set(sym, ui);
                    }
                    iplay += 1;
                }

            }
            while iplay < self.max_space {
                if let Some((counter, &sym)) = change_table_space_iter.next() {
                    let f = format!("{} {} {}",
                                    self.appdata.text_changeto(),
                                    (iplay) + 1,
                                    self.appdata.text_playergame());
                    let but = pad_text_button::Button::new(4)
                        .label(&f)
                        .label_color(style.label_color((&ui.theme)))
                        .right_from(state.ids.players_text,
                                    (counter as f64) * change_table_space_but_w)
                        .w_h(change_table_space_but_w, 0.8 * h)
                        .set(sym, ui);

                    for _i in but {
                        (*self.change_table_space_closure)(iplay + 1);
                    }
                    iplay += 1;
                }

            }
        }

        Some(())
    }
}
