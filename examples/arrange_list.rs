#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable, Rect, Borderable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;
use conrod::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::image_hover::Hoverable;
use cardgame_widgets::custom_widget::arrange_list::{ArrangeList, ItemWidget};
use cardgame_widgets::sprite::{spriteable_rect, SpriteInfo};
use std::time::Instant;

pub struct ImageHoverable(Image, Option<Image>, Option<Image>);
impl Hoverable for ImageHoverable {
    fn idle(&self) -> Image {
        self.0
    }
    fn hover(&self) -> Option<Image> {
        self.1
    }
    fn press(&self) -> Option<Image> {
        self.2
    }
}

widget_ids! {
    pub struct Ids {
         master,
         arrange_view,
    }
}
pub struct App {
    hash: Vec<color::Color>,
    blow_up: Option<usize>,
}
#[derive(Clone)]
pub enum ConrodMessage {
    Event(Instant, conrod::event::Input),
    Thread(Instant),
}
fn main() {
    let window = glutin::WindowBuilder::new();
    let context =
        glium::glutin::ContextBuilder::new()
            .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs, (3, 0)));
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    // construct our `Ui`.
    let (screen_w, screen_h) = display.get_framebuffer_dimensions();
    let mut ui = conrod::UiBuilder::new([screen_w as f64, screen_h as f64]).build();

    let rust_logo = load_image(&display, "images/rust.png");
    let button_logo = load_image(&display, "images/arrows_but.png");
    let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut demo_text_edit = "Click here !".to_owned();
    let mut last_update = std::time::Instant::now();
    let mut c = 0;
    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);
    let button_logo = image_map.insert(button_logo);
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(800);
    let mut app = App {
        hash: vec![color::DARK_YELLOW, color::YELLOW, color::DARK_BLUE, color::LIGHT_PURPLE],
        blow_up: None,
    };

    'render: loop {
        let mut to_break = false;
        let mut to_continue = false;
        events_loop.poll_events(|event| {
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::Closed |
                            glium::glutin::WindowEvent::KeyboardInput {
                                input: glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => {to_break=true;}
                        _ => (),
                    }
                }
                _ => {}
            }
            let input = match conrod::backend::winit::convert_event(event.clone(), &display) {
                None => {
                    to_continue = true;
                }
                Some(input) => {
                    let d = std::time::Instant::now();
                    if let event::Input::Text(s) = input.clone() {
                        if s != String::from("") {
                            captured_event = Some(ConrodMessage::Event(d, input));
                        }
                    } else {
                        captured_event = Some(ConrodMessage::Event(d, input));
                    }

                }
            };
        });
        if to_break {
            break 'render;
        }
        if to_continue {
            continue;
        }
        match captured_event {
            Some(ConrodMessage::Event(d, ref input)) => {
                if let Some(ConrodMessage::Event(oldd, ref oldinput)) = old_captured_event {
                    if oldinput.clone() != input.clone() {
                        ui.handle_event(input.clone());
                    }
                }
                if let None = old_captured_event {
                    ui.handle_event(input.clone());
                }
                old_captured_event = Some(ConrodMessage::Event(d, input.clone()));
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &mut app, rust_logo, button_logo);

            }
            Some(ConrodMessage::Thread(t)) => {
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &mut app, rust_logo, button_logo);
            }
            None => {
                let now = std::time::Instant::now();
                let duration_since_last_update = now.duration_since(last_update);
                if duration_since_last_update < sixteen_ms {
                    std::thread::sleep(sixteen_ms - duration_since_last_update);
                }
                let t = std::time::Instant::now();
                captured_event = Some(ConrodMessage::Thread(t));
            }
        }

        let primitives = ui.draw();
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(&display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
        last_update = std::time::Instant::now();
    }
}

fn set_widgets(ui: &mut conrod::UiCell,
               ids: &mut Ids,
               _app: &mut App,
               rust_logo: conrod::image::Id,
               button: conrod::image::Id) {
    //zz
    let b_s = button_sprite();
    let l_0 = spriteable_rect(b_s, 4.0);
    let l_1 = spriteable_rect(b_s, 6.0);
    let t_0 = spriteable_rect(b_s, 0.0);
    let t_1 = spriteable_rect(b_s, 2.0);
    let r_0 = spriteable_rect(b_s, 1.0);
    let r_1 = spriteable_rect(b_s, 3.0);
    let b_0 = spriteable_rect(b_s, 5.0);
    let b_1 = spriteable_rect(b_s, 7.0);
    let left_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(l_0.0, l_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(l_1.0, l_1.1))),
                       None);
    let top_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(t_0.0, t_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(t_1.0, t_1.1))),
                       None);
    let right_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(r_0.0, r_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(r_1.0, r_1.1))),
                       None);
    let btm_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(b_0.0, b_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(b_1.0, b_1.1))),
                       None);
    widget::Canvas::new().color(color::WHITE).set(ids.master, ui);
    let (_, _, scrollbar) = ArrangeList::new(&mut _app.hash,
                                             &mut _app.blow_up,
                                             Box::new(move |v| {
        let i_h_struct = ImageHoverable(Image::new(rust_logo.clone()), None, None);
        ItemWidget::new(i_h_struct).color(v).border_color(color::YELLOW).border(20.0)
    }),
                                             200.0)
            .w_h(400.0, 400.0)
            .color(color::RED)
            .middle_of(ids.master)
            .left_arrow(left_arrow_z)
            .top_arrow(top_arrow_z)
            .right_arrow(right_arrow_z)
            .bottom_arrow(btm_arrow_z)
            .set(ids.arrange_view, ui);
    if let Some(s) = scrollbar {
        s.set(ui);
    }

}
fn load_image(display: &glium::Display, path: &str) -> glium::texture::Texture2d {
    let rgba_image = support::assets::load_image(path).to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                       image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
fn button_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0), //left corner of first
        num_in_row: 4,
        num_in_col: 2,
        w_h: (200.0, 200.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
