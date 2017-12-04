#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable, Rect};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;
use conrod::widget::Canvas;
use std::time::Instant;
#[derive(Clone,PartialEq,Debug)]
enum Gamestate {
    Green,
    Red,
}
widget_ids! {
    pub struct Ids {
         master,
         body,
         footer,
         footer_image,
         overlay,
         overlaytop,
         overlaybtm
    }
}
pub struct App {
    gamestate: Gamestate,
    frame: u32,
    overlay: bool,
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

    let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut demo_text_edit = "Click here !".to_owned();
    let mut last_update = std::time::Instant::now();
    let mut c = 0;
    let rust_logo = load_image(&display, "images/green.png");
    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(800);
    let mut app = App {
        gamestate: Gamestate::Green,
        frame: 0,
        overlay: true,
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
                set_widgets(&mut ui, &mut ids, &mut app, rust_logo);

            }
            Some(ConrodMessage::Thread(t)) => {
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &mut app, rust_logo);
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
               rust_logo: conrod::image::Id) {

    match &_app.gamestate {
        &Gamestate::Green => {

            Canvas::new()
                   .flow_down(&[(ids.body,
                                 Canvas::new().color(color::BLUE)),
                                (ids.footer,
                                 Canvas::new()
                                     .color(color::DARK_GREEN)
                                     .length(200.0))])
                   .color(color::LIGHT_BLUE)
                 /* .close_icon(rust_logo)
                   .frame_rate(30)
                   */
                   .set(ids.master, ui);
            /*       .is_done() {
                _app.gamestate = Gamestate::Red;
            }*/

            for k in widget::Button::image(rust_logo)
                    .w_h(90.0, 90.0)
                    .middle_of(ids.footer)
                    .set(ids.footer_image, ui) {
                _app.overlay = true;
            }

        }
        &Gamestate::Red => {
            Canvas::new()
                   .flow_down(&[(ids.body,
                                 Canvas::new().color(color::BLUE)),
                                (ids.footer,
                                 Canvas::new()
                                     .color(color::RED)
                                     .length(200.0))])
                   .color(color::LIGHT_BLUE)
                //   .frame_rate(30)
                 //  .close_icon(rust_logo)
                   .set(ids.master, ui);
            //.is_done() {_app.gamestate = Gamestate::Green;};
            for k in widget::Button::image(rust_logo)
                    .w_h(90.0, 90.0)
                    .middle_of(ids.footer)
                    .set(ids.footer_image, ui) {
                _app.overlay = true;
            }
        }

    }
    if _app.overlay {
        Canvas::new()
                   // .pad(200.0)
                    .wh_of(ids.master)
                    .middle_of(ids.master)
                   .flow_down(&[(ids.overlaytop,
                                 Canvas::new().color(color::GREY).length(200.0)),
                                (ids.overlaybtm,
                                 Canvas::new()
                                     .color(color::RED))])
                   .color(color::TRANSPARENT)
                 //  .close_icon(rust_logo)
                  // .close_icon_src_rect(Rect::from_corners([27.0,33.0], [117.0,100.0]))
                //   .frame_rate(30)
                   .set(ids.overlay, ui);
        /*.is_done(){
                _app.overlay =false;        
                   }*/
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
