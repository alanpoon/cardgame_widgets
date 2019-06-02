use custom_widget::arrange_list::{Hoverable, ImageHover, Arrangeable, TimesClicked};
use conrod_core::{widget, Positionable, Widget, Sizeable, Color, Scalar, Borderable, Colorable, UiCell,
             Rect};
use conrod_core::widget::Rectangle;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<H: Hoverable> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image: H,
    pub bordered: bool,
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the Button's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the Image
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
}

widget_ids! {
    struct Ids {
        background,
        rect,
        image,
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<H> ItemWidget<H>
    where H: Hoverable
{
    /// Create a button context to be built upon.
    pub fn new(image: H) -> Self {
        ItemWidget {
            image: image,
            common: widget::CommonBuilder::default(),
            bordered: false,
            style: Style::default(),
        }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<H> Widget for ItemWidget<H>
    where H: Hoverable
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = TimesClicked;

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
        let (_, _, w, h) = rect.x_y_w_h();
        let border = if self.bordered {
            self.style.border(ui.theme())
        } else {
            0.0
        };
        rectangle_fill(id,
                       state.ids.background,
                       rect,
                       self.style.color(&ui.theme),
                       ui);
        if self.bordered {
            let border_color = self.style.border_color(ui.theme());
            let _style = widget::line::Style {
                maybe_pattern: None,
                maybe_color: Some(border_color),
                maybe_thickness: Some(border),
                maybe_cap: None,
            };
            Rectangle::outline_styled([w, h],_style).middle_of(id)
            .parent(id)
            //.graphics_for(id)
            .set(state.ids.rect, ui);
        }

        ImageHover::new(self.image)
            .middle_of(id)
            .padded_wh_of(id, border)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.image, ui)
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
impl<H> Arrangeable for ItemWidget<H>
    where H: Hoverable
{
    fn selectable(mut self) -> Self {
        self.bordered = true;
        self
    }
}
impl<H> Colorable for ItemWidget<H>
    where H: Hoverable
{
    builder_method!(color { style.color = Some(Color) });
}
impl<H> Borderable for ItemWidget<H>
    where H: Hoverable
{
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
