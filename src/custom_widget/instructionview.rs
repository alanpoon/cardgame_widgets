use conrod::{self, widget, Positionable, Widget, image, Sizeable, color, text, Labelable};
use custom_widget::animated_button;
use sprite::SpriteInfo;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct InstructionView<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub transparent: image::Id,
    pub button: image::Id,
    pub button_sprite: SpriteInfo,
    pub dist_arrow_from_center: [f64; 2],
    pub transparent_rect: [f64; 4],
    pub instruction: Option<&'a str>,
    pub next: &'a str,
    pub dif_frame: Option<i32>,
    pub ovaldim: [f64; 2],
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

widget_ids! {
    struct Ids {
        oval,
        transparent,
        arrow_head,
        arrow_body,
        frame,
        instruction,
        next,
    }
}

/// Represents the unique, cached state for our InstructionView widget.
pub struct State {
    ids: Ids,
}

impl<'a> InstructionView<'a> {
    /// Create a button context to be built upon.
    pub fn new(transparent: image::Id,
               button: image::Id,
               button_sprite: SpriteInfo,
               dist_arrow_from_center: [f64; 2],
               transparent_rect: [f64; 4],
               ovaldim: [f64; 2],
               dif_frame: Option<i32>,
               instruction: Option<&'a str>,
               next: &'a str)
               -> Self {
        InstructionView {
            dist_arrow_from_center: dist_arrow_from_center,
            transparent_rect: transparent_rect,
            transparent: transparent,
            button: button,
            button_sprite: button_sprite,
            dif_frame: dif_frame,
            ovaldim: ovaldim,
            instruction: instruction,
            next: next,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for InstructionView<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `true` when clicked, otherwise `None`.
    type Event = bool;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { state, rect, ui, .. } = args;

        let (_x, _y, w, h) = rect.x_y_w_h();
        widget::Image::new(self.transparent)
            .w_h(w * 1.2, h * 1.2)
            .middle()
            .set(state.ids.transparent, ui);
        widget::Rectangle::fill_with([0.6 * w, h * 0.3], color::LIGHT_YELLOW)
            .mid_left()
            .set(state.ids.frame, ui);

        let j = [self.dist_arrow_from_center[0], self.dist_arrow_from_center[1] + 0.15 * h];
        if let Some(dif_frame) = self.dif_frame {
            match dif_frame {
                0 => {
                    widget::Oval::outline_styled([self.ovaldim[0] * 1.2, self.ovaldim[1]],
                                                 widget::line::Style::new().thickness(5.0))
                            .xy_relative_to(state.ids.frame, j)
                            .set(state.ids.oval, ui);
                }
                1 => {
                    widget::Oval::outline_styled(self.ovaldim,
                                                 widget::line::Style::new().thickness(5.0))
                            .xy_relative_to(state.ids.frame, j)
                            .set(state.ids.oval, ui);
                }
                _ => {}
            }

        }
        let start = [-0.3 * w, 0.15 * h];
        let end = [-0.3 * w + self.dist_arrow_from_center[0],
                   0.15 * h + self.dist_arrow_from_center[1]];
        widget::Line::new(start, end).thickness(10.0).set(state.ids.arrow_body, ui);
        if let Some(instruct) = self.instruction {
            let mut a1 = 20;
            let mut l1 = 2.5;
            while text::height(3, a1, l1) > 0.25 * h {
                a1 -= 1;
                l1 -= 0.5;
            }
            widget::Text::new(instruct)
                .font_size(a1)
                .top_left_with_margins_on(state.ids.frame, 0.1 * h, 10.0)
                .w(0.6 * w)
                .h(0.25 * h)
                .set(state.ids.instruction, ui);
        }

        let _style = self.button_sprite;
        animated_button::AnimatedButton::image(self.button)
            .label(self.next)
            .normal_rect(_style.src_rect(7.0))
            .hover_rect(_style.src_rect(7.0 + 1.0))
            .press_rect(_style.src_rect(7.0 + 2.0))
            .down_from(state.ids.frame, 0.0)
            .w(100.0)
            .h(70.0)
            .set(state.ids.next, ui)
            .was_clicked()
    }
}
