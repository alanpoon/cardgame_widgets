#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;

use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image::Button;
use cardgame_widgets::sprite::SpriteInfo;
use std::time::Instant;

widget_ids! {
    pub struct Ids {
         master,
         wraplist,
         floating_a,
         exit_id
    }
}
pub struct App {
    hash: Vec<color::Color>,
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
    let green_logo = load_image(&display, "images/green.png");
    let spinner_logo = load_image(&display, "images/download.png");
    let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut demo_text_edit = "Click here !".to_owned();
    let mut last_update = std::time::Instant::now();
    let mut c = 0;
    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);
    let green_logo = image_map.insert(green_logo);
    let spinner_logo = image_map.insert(spinner_logo);
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(1000);
    let mut app = App {
        hash: vec![color::DARK_YELLOW, color::YELLOW, color::DARK_BLUE, color::LIGHT_PURPLE],
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
                set_widgets(&mut ui,
                            &mut ids,
                            &mut app,
                            rust_logo,
                            green_logo,
                            spinner_logo);

            }
            Some(ConrodMessage::Thread(t)) => {
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui,
                            &mut ids,
                            &mut app,
                            rust_logo,
                            green_logo,
                            spinner_logo);
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
               app: &mut App,
               rust_logo: conrod::image::Id,
               green_logo: conrod::image::Id,
               spinner_logo: conrod::image::Id) {
    widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
    // let j = widget::Canvas::new().w_h(100.0, 300.0);
    widget::Rectangle::fill([200.0, 200.0])
        .top_right_of(ids.master)
        .color(color::GREEN)
        .set(ids.exit_id, ui);
    let spinner_rect = spinner_sprite();
    let (exitable,_scroll) = DragDropList::new(&mut app.hash,
                                     Box::new(move |v| {
        let j = Button::image(rust_logo.clone())
            .toggle_image(green_logo.clone())
            .spinner_image(spinner_logo.clone(),spinner_rect)
            .w_h(100.0, 300.0);
        j.color(v)
    }),
                                     50.0)
            .wh([400.0, 400.0])
            .color(color::RED)
            .exit_id(ids.exit_id)
            .middle_of(ids.master)
            .set(ids.wraplist, ui);
    if let Some(exitable) = exitable {
        println!("exitable {:?}", exitable);
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
pub fn spinner_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0),
        num_in_row: 12,
        num_in_col: 4,
        w_h: (100.0, 100.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}