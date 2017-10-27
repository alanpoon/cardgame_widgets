#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;

use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable, Borderable, Labelable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;
use conrod::widget::envelope_editor::EnvelopePoint;
use cardgame_widgets::custom_widget::dragdrop_list;
use std::time::Instant;

widget_ids! {
    pub struct Ids {
         master,
         wraplist,
         circle_position,
         circle
    }
}
pub struct App {
    circle_pos: conrod::Point,
    ddl_colors: Vec<String>,
    /// The currently selected DropDownList color.
    ddl_color: conrod::Color,
    border_width: f64,
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
    let mut last_update_sys = std::time::SystemTime::now();
    let mut c = 0;
    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(100);
    let mut app = App {
        ddl_colors: vec!["Black".to_string(),
                         "White".to_string(),
                         "Red".to_string(),
                         "Green".to_string(),
                         "Blue".to_string()],
        ddl_color: conrod::color::PURPLE,
        circle_pos: [-50.0, 110.0],
        border_width: 1.0,
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
                set_widgets(&mut ui, &mut ids, &mut app);

            }
            Some(ConrodMessage::Thread(t)) => {
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &mut app);
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

fn set_widgets(ui: &mut conrod::UiCell, ids: &mut Ids, app: &mut App) {
    widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
    // Draw an xy_pad.
    for (x, y) in widget::XYPad::new(app.circle_pos[0], -75.0, 75.0, // x range.
                                         app.circle_pos[1], 95.0, 245.0) // y range.
            .w_h(150.0, 150.0)
            .middle() // Align to the bottom of the last toggle_matrix element.
            .color(app.ddl_color)
            .border(app.border_width)
            .border_color(color::WHITE)
            .label("Circle Position")
            .label_color(app.ddl_color.plain_contrast().alpha(0.5))
            .set(ids.circle_position, ui) {
        app.circle_pos[0] = x;
        app.circle_pos[1] = y;
    }

    // Draw a circle at the app's circle_pos.
    widget::Circle::fill(15.0)
        .xy_relative_to(ids.circle_position, app.circle_pos)
        .color(app.ddl_color)
        .set(ids.circle, ui);

}

//conrod::position::Point
fn rearrange<T: Clone>(selected_i: usize,
                       corrected_i: usize,
                       hash: &mut [Option<([f64; 2], T)>; 25]) {
    println!("select{}, corrected{}", selected_i, corrected_i);
    let hash_c = hash.clone();
    for _i in 0..hash.len() {
        if _i == corrected_i {
            hash[_i] = match (&hash_c[selected_i], &hash_c[_i]) {
                (&Some((_, ref a_s)), &Some((pos, _))) => Some((pos, a_s.clone())),
                _ => None,
            };
        }
        if selected_i < corrected_i {
            //moved backward ____S__->__C
            if (_i < corrected_i) & (_i >= selected_i) {
                println!("move backward");
                // ____S~~~~C;
                hash[_i] = match (&hash_c[_i + 1], &hash_c[_i]) {
                    (&Some((_, ref a_s)), &Some((pos, _))) => Some((pos, a_s.clone())),
                    _ => None,
                };
            }
        } else if selected_i > corrected_i {
            //moved foward _____C__<-S
            if (_i <= selected_i) & (_i > corrected_i) {
                println!("move forward");
                // ____C~~~S
                hash[_i] = match (&hash_c[_i - 1], &hash_c[_i]) {
                    (&Some((_, ref a_s)), &Some((pos, _))) => Some((pos, a_s.clone())),
                    _ => None,
                };
            }
        }

    }
}
