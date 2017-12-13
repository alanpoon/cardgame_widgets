use conrod::{self, widget, Positionable, Widget, Sizeable, text, Labelable, Colorable, color};
use custom_widget::animated_canvas;
use custom_widget::pad_text_button;
pub trait PromptSendable {
    fn send(&self, msg: String);
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct PromptView<'a, PS>
    where PS: PromptSendable + Clone + 'a
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
    /// Color of the bg_color.
    #[conrod(default = "color::BLACK.with_alpha(0.4)")]
    pub bg_color: Option<conrod::Color>,
    /// Dimension of the prompt
    #[conrod(default = "[500.0,300.0]")]
    pub prompt_wh: Option<[f64; 2]>,
    /// Item_size of the button option
    #[conrod(default = "200.0")]
    pub item_size: Option<f64>,
}

widget_ids! {
    struct Ids {
        canvas,
        body,
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
    where PS: PromptSendable + Clone + 'a
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
        pub bg_color { style.bg_color = Some(conrod::Color) }
        pub prompt_wh { style.prompt_wh = Some([f64;2]) }
        pub item_size{style.item_size = Some(f64)}
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, PS> Widget for PromptView<'a, PS>
    where PS: PromptSendable + Clone + 'a
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

        let (_x, _y, _w, _h) = rect.x_y_w_h();
        let mut should_close = false;
        let bg_color = self.style.bg_color(&ui.theme);
        let prompt_wh = self.style.prompt_wh(&ui.theme);
        let item_size = self.style.item_size(&ui.theme);
        if let &mut Some(ref _z) = self.prompt {
            animated_canvas::Canvas::new()
                .color(bg_color)
                .wh_of(id)
                .middle_of(id)
                .frame_rate(30)
                .set(state.ids.canvas, ui);
            let num = _z.2.len();
            let label_font_size = self.style.label_font_size(&ui.theme);
            let color = self.style.color(&ui.theme);
            widget::Rectangle::fill_with(prompt_wh, color)
                .middle_of(state.ids.canvas)
                .set(state.ids.rect, ui);
            widget::Text::new(&_z.1)
                .w(_z.0 * prompt_wh[0])
                .h(0.5 * prompt_wh[1])
                .font_size(label_font_size)
                .color(color.plain_contrast())
                .mid_top_with_margin_on(state.ids.rect, 0.1 * prompt_wh[1])
                .set(state.ids.prompt, ui);
            let (mut items, _) = widget::List::flow_right(num)
                .down_from(state.ids.prompt, 0.0)
                .align_middle_x_of(state.ids.rect)
                .item_size(item_size)
                .w(prompt_wh[0] - _z.0)
                .set(state.ids.list, ui);
            let mut vec_iter = _z.2.iter();
            while let (Some(&(ref label, ref closure)), Some(ref item)) =
                (vec_iter.next(), items.next(ui)) {
                let d = pad_text_button::Button::new()
                    .w((prompt_wh[0] - _z.0) / (num as f64))
                    .h(0.3 * prompt_wh[1])
                    .label(&label)
                    .label_color(color.plain_contrast())
                    .label_font_size(label_font_size);
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
        //animated canvas should be here
        Some(())
    }
}
