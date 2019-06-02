use conrod_core::{self, widget, Positionable, Widget, Sizeable, color, text, Labelable,image};
use custom_widget::animated_button;
use sprite::SpriteInfo;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ErrorDialogue<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub button_image: image::Id,
    pub buttons_sprite_label: Vec<(SpriteInfo, &'a str, Box<Fn() + 'a>)>,
    pub prompt: (f64, &'a str), //width%,text
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
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
}

widget_ids! {
    struct Ids {
        prompt,
        list,
    }
}

/// Represents the unique, cached state for our PromptView widget.
pub struct State {
    ids: Ids,
}

impl<'a> PromptView<'a> {
    /// Create a button context to be built upon.
    pub fn new(button_image: image::Id,
               buttons_sprite_label: Vec<(SpriteInfo, &'a str, Box<Fn() + 'a>)>,
               prompt: (f64, &'a str))
               -> Self {
        PromptView {
            button_image: button_image,
            prompt: prompt,
            buttons_sprite_label: buttons_sprite_label,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod_core::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for PromptView<'a> {
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
        let widget::UpdateArgs { id, state, rect, mut ui, .. } = args;
        let num = self.buttons_sprite_label.len();
        let (x, y, w, h) = rect.x_y_w_h();
        let mut f1 = 16;
        let mut l1 = 2.0;

        while text::height(2, f1, l1) > 0.5 * h {
            f1 -= 1;
            l1 -= 0.2;
        }
        widget::Text::new(self.prompt.1)
            .w(self.prompt.0 * w)
            .h(0.5 * h)
            .font_size(f1)
            .line_spacing(l1)
            .top_left_with_margins_on(id, 0.0, 0.0)
            .set(state.ids.prompt, ui);
        let (mut events, _) = widget::list_select::ListSelect::single(num)
            .flow_right()
            .down_from(state.ids.prompt, 0.0)
            .item_size(40.0)
            .w(w - self.prompt.0)
            .set(state.ids.list, ui);
        let mut vec_iter = self.buttons_sprite_label.iter();
            while let (Some(&(z, label, ref closure)), Some(ref event)) =
                (vec_iter.next(), events.next(ui, |_| false)) {
                use conrod_core::widget::list_select::Event;
                match event {
                    &Event::Item(item) => {
                        let button_index = 1.0;
                        let d = animated_button::AnimatedButton::image(self.button_image)
                            .label(label)
                            .label_font_size(16)
                            .label_color(color::BLACK)
                            .w((w - self.prompt.0) / (num as f64))
                            .h(0.3 * h)
                            .normal_rect(z.src_rect(button_index))
                            .hover_rect(z.src_rect(button_index + 1.0))
                            .press_rect(z.src_rect(button_index + 2.0));
                        let dj = item.set(d, ui);
                        if dj.was_clicked() {
                            (closure)();
                        }
                    }
                    //&Event::Selection(idx) => {}
                    _ => {}

                }
            }

        Some(())
    }
}
