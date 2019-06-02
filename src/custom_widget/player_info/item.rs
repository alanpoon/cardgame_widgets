use conrod_core::{widget, Positionable, Widget, Sizeable, text, Color, Colorable, Scalar};
use conrod_core::widget::primitive::image::Image;
use conrod_core::widget::Rectangle;
use text::get_font_size_wh;
#[derive(Clone)]
pub struct IconStruct(pub Image, pub String, pub String);
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct Icon {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub icon: IconStruct,
    /// See the Style struct below.
    style: Style,
    pub bordered: bool,
}

#[derive(Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// The color of the AnimatedButton's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The ID of the font used to display the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<text::font::Id>>,
    /// Width of the border surrounding the Image
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
}

widget_ids! {
    struct Ids {
        border,
        rect,
        image,
        label
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}
impl Icon {
    /// Create a button context to be built upon.
    pub fn new(icon: IconStruct) -> Self {
        Icon {
            icon: icon,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            bordered: false,
        }
    }
    builder_methods!{
        pub label_color { style.label_color = Some(Color) }
        pub border { style.border = Some(Scalar) }
        pub border_color { style.border_color = Some(Color) }
    }
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }
    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl Widget for Icon {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = ();

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

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (_, _, w, _) = rect.x_y_w_h();
        let h = w / 2.0;
        let border = if self.bordered {
            self.style.border(ui.theme())
        } else {
            0.0
        };
        if self.bordered {
            let border_color = self.style.border_color(ui.theme());
            let _style = widget::line::Style {
                maybe_pattern: None,
                maybe_color: Some(border_color),
                maybe_thickness: Some(border),
                maybe_cap: None,
            };
            Rectangle::outline_styled([w * 0.97, h], _style)
                .middle_of(id)
                .parent(id)
                .set(state.ids.rect, ui);
        }
        self.icon
            .0
            .w_h(h * 0.8, h * 0.8)
            .mid_left_of(id)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.image, ui);
        let fontsize = get_font_size_wh(h * 1.2, h * 1.2, &self.icon.1);
        // let font_id = self.style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
        widget::Text::new(&self.icon.1)
            .w_h(h, h)
            .font_size(fontsize)
            .right_from(state.ids.image, 0.0)
            .parent(id)
            .graphics_for(id)
            .left_justify()
            //.and_then(font_id, widget::Text::font_id)
            .color(self.style.label_color(&ui.theme))
            .set(state.ids.label, ui);

    }
}
