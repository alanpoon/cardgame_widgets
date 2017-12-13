use conrod::{self, widget, Positionable, Widget, Sizeable, text, Labelable, Colorable};
pub trait PromptSender {
    fn send(&self, msg: String);
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct PromptView<'a, PS>
    where PS: PromptSender + Clone + 'a
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub prompt: &'a mut Option<(f64, String, Vec<(String, Box<Fn(PS)>)>)>, //width%,text
    pub promptsender: PS,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

widget_ids! {
    struct Ids {
        rect,
        prompt,
        list,
    }
}

/// Represents the unique, cached state for our PromptView widget.
pub struct State {
    ids: Ids,
}

impl<'a, PS> PromptView<'a, PS>
    where PS: PromptSender + Clone + 'a
{
    /// Create a button context to be built upon.
    pub fn new(prompt: &'a mut Option<(f64, String, Vec<(String, Box<Fn(PS)>)>)>,
               promptsender: PS)
               -> Self {
        PromptView {
            prompt: prompt,
            promptsender: promptsender,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
    builder_methods!{
        pub color { style.color = Some(conrod::Color) }
        pub label_font_size{style.label_font_size = Some(conrod::FontSize)}
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, PS> Widget for PromptView<'a, PS>
    where PS: PromptSender + Clone + 'a
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
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;

        let (_x, _y, w, h) = rect.x_y_w_h();
        let mut f1 = 16;
        let mut l1 = 2.0;
        let mut should_close = false;
        if let &mut Some(ref _z) = self.prompt {
            let num = _z.2.len();
            while text::height(2, f1, l1) > 0.5 * h {
                f1 -= 1;
                l1 -= 0.2;
            }
            let color = self.style.color(&ui.theme);
            widget::Rectangle::fill_with([w, h], color).middle_of(id).set(state.ids.rect, ui);
            widget::Text::new(&_z.1)
                .w(_z.0 * w)
                .h(0.5 * h)
                .color(color.plain_contrast())
                .font_size(f1)
                .line_spacing(l1)
                .top_left_with_margins_on(id, 0.0, 0.0)
                .set(state.ids.prompt, ui);
            let (mut items, _) = widget::List::flow_right(num)
                .down_from(state.ids.prompt, 0.0)
                .item_size(100.0)
                .w(w - _z.0)
                .set(state.ids.list, ui);
            let mut vec_iter = _z.2.iter();
            while let (Some(&(ref label, ref closure)), Some(ref item)) =
                (vec_iter.next(), items.next(ui)) {

                let d = widget::Button::new()
                    .w((w - _z.0) / (num as f64))
                    .h(0.3 * h)
                    .label(&label)
                    .label_color(color.plain_contrast())
                    .label_font_size(self.style.label_font_size(&ui.theme));
                let dj = item.set(d, ui);
                for _ in dj {
                    (*closure)(self.promptsender.clone());

                    should_close = true;
                }

            }
        }
        if should_close {
            *self.prompt = None;
        }

        Some(())
    }
}
