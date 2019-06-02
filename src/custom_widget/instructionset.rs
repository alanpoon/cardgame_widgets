use conrod_core::{self, widget, Positionable, Widget, Labelable, Colorable, Sizeable};
use conrod_core::widget::{Rectangle, Oval};
use conrod_core::widget::primitive::shape::oval::Full;
use conrod_core::widget::button::{Button, Flat};
pub trait Instructable<'a> {
    fn label(&self) -> &'a str;
    fn rect(&self, [f64; 2]) -> Rectangle;
    fn button(&self, [f64; 2]) -> Button<Flat>;
    fn oval_one(&self, [f64; 2]) -> Option<Oval<Full>>;
    fn oval_two(&self, [f64; 2]) -> Option<Oval<Full>>;
}
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct InstructionSet<'a, I>
    where I: Instructable<'a> + 'a
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub instructions: &'a Vec<I>, //str,[l,t,w,h of rect],Some([l,t,w,h of oval])
    pub instruction_cache: &'a mut usize,
    pub next: &'a str,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub button_color: Option<conrod_core::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod_core::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod_core::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod_core::text::font::Id>>,
    /// Specify a parent_id
    #[conrod(default="None")]
    pub parent_id: Option<Option<widget::Id>>,
}

widget_ids! {
    struct Ids {
        oval,
        frame,
        instruction,
        next,
    }
}

/// Represents the unique, cached state for our InstructionSet widget.
pub struct State {
    ids: Ids,
    frame: usize,
}

impl<'a, I> InstructionSet<'a, I>
    where I: Instructable<'a> + 'a
{
    /// Create a button context to be built upon.
    pub fn new(instructions: &'a Vec<I>, instruction_cache: &'a mut usize, next: &'a str) -> Self {
        InstructionSet {
            instructions: instructions,
            instruction_cache: instruction_cache,
            next: next,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod_core::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
    pub fn parent_id(mut self, parent_id: widget::Id) -> Self {
        self.style.parent_id = Some(Some(parent_id));
        self
    }
    builder_methods!{
        pub button_color { style.button_color = Some(conrod_core::Color) }
        pub label_color{style.label_color = Some(conrod_core::Color)}
        pub label_font_size{style.label_font_size = Some(conrod_core::FontSize)}
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, I> Widget for InstructionSet<'a, I>
    where I: Instructable<'a> + 'a
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `true` when clicked, otherwise `None`.
    type Event = bool;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            frame: 0,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        let len = self.instructions.len();
        let mut print = true;
        let style = self.style;
        let ((_x, _y, w, h), _parent_id) = if let Some(Some(_parent_id)) = style.parent_id {
            (ui.rect_of(_parent_id).unwrap().x_y_w_h(), _parent_id)
        } else {
            (rect.x_y_w_h(), id)
        };

        if let Some(_inst) = self.instructions.get(self.instruction_cache.clone()) {
            let (_label, _rect, _button, _oval_one, _oval_two) = (_inst.label(),
                                                                  _inst.rect([w, h]),
                                                                  _inst.button([w, h]),
                                                                  _inst.oval_one([w, h]),
                                                                  _inst.oval_two([w, h]));
            _rect.set(state.ids.frame, ui);
            let (_rx, _ry, _rw, _rh) = ui.rect_of(state.ids.frame).unwrap().x_y_w_h();
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::TextEdit::new(_label)
                .font_id(font_id.unwrap())
                .color(style.label_color(&ui.theme))
                .font_size(style.label_font_size(&ui.theme))
                .padded_w_of(state.ids.frame, 0.1 * _rw)
                .padded_h_of(state.ids.frame, 0.1 * _rh)
                .top_left_with_margins_on(state.ids.frame, 0.1 * _rh, 0.1 * _rw)
                .set(state.ids.instruction, ui);

            let j = _button.color(style.button_color(&ui.theme))
                .label(self.next)
                .parent(state.ids.frame)
                .set(state.ids.next, ui);
            for _ in j {
                if self.instruction_cache.clone() + 1 == len {
                    print = false;
                } else {
                    *self.instruction_cache += 1;
                }

            }
            if let (Some(_oval_one), Some(_oval_two)) = (_oval_one, _oval_two) {
                if state.frame == 0 {
                    _oval_one.set(state.ids.oval, ui);
                    state.update(|state| state.frame += 1);
                } else {
                    _oval_two.set(state.ids.oval, ui);
                    state.update(|state| state.frame = 0);
                }
            }

        }
        print
    }
}
