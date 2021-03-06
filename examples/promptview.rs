#[macro_use]
extern crate conrod;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod_core::{widget, color, Colorable, Widget, Positionable, Sizeable};
use conrod_core::backend::glium::glium::{self, glutin, Surface};
use conrod_core::event;

use cardgame_widgets::custom_widget::promptview::{PromptView, PromptSendable};
use std::time::Instant;
use std::sync::mpsc::Sender;
widget_ids! {
    pub struct Ids {
         master,
         body,
         footer,
         promptview,
         button_body
    }
}
pub struct App<PS>
    where PS: PromptSendable + Clone
{
    prompt: Option<(f64, String, Vec<(String, Box<Fn(PS)>)>)>,
}
#[derive(Clone)]
pub struct PromptSender(Sender<String>);
impl PromptSendable for PromptSender {
    fn send(&self, msg: String) {
        self.0.send(msg).unwrap();
    }
}

#[derive(Clone)]
pub enum ConrodMessage {
    Event(Instant, conrod_core::event::Input),
    Thread(Instant),
}
fn main() {
    let window = glutin::WindowBuilder::new();
    let context =
        glium::glutin::ContextBuilder::new()
            .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs, (3, 0)));
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod_core::backend::glium::Renderer::new(&display).unwrap();
    // construct our `Ui`.
    let (screen_w, screen_h) = display.get_framebuffer_dimensions();
    let mut ui = conrod_core::UiBuilder::new([screen_w as f64, screen_h as f64]).build();
    ui.fonts.insert(support::assets::load_font("fonts/NotoSans/NotoSans-Regular.ttf"));
    //let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut last_update = std::time::Instant::now();
    let image_map: conrod_core::image::Map<glium::texture::Texture2d> = conrod_core::image::Map::new();

    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(1000);
    let (test_tx, test_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || loop {
                           while let Ok(s) = test_rx.try_recv() {
                               println!("s is {:?}", s);
                           }
                       });

    let promptsender = PromptSender(test_tx);
    let mut app = App::<PromptSender> {
        prompt: Some((0.5,
                      "asdsadasasd".to_owned(),
                      vec![("instruction 1".to_owned(),
                            Box::new(|ps| {
                                         let f = format!("{{'greedcommand':1}}");

                                         ps.send(f);
                                     })),
                           ("instruction 2".to_owned(),
                            Box::new(|ps| {
                                         let f = format!("{{'greedcommand':2}}");

                                         ps.send(f);
                                     }))])),
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
            let _input = match conrod_core::backend::winit::convert_event(event.clone(), &display) {
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
                if let Some(ConrodMessage::Event(_oldd, ref oldinput)) = old_captured_event {
                    if oldinput.clone() != input.clone() {
                        ui.handle_event(input.clone());
                    }
                }
                if let None = old_captured_event {
                    ui.handle_event(input.clone());
                }
                old_captured_event = Some(ConrodMessage::Event(d, input.clone()));
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &mut app, promptsender.clone());

            }
            Some(ConrodMessage::Thread(_t)) => {
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &mut app, promptsender.clone());
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

fn set_widgets(ui: &mut conrod_core::UiCell,
               ids: &mut Ids,
               app: &mut App<PromptSender>,
               promptsender: PromptSender) {
    widget::Canvas::new()
        .color(color::TRANSPARENT)
        .flow_down(&[(ids.body, widget::Canvas::new().color(color::DARK_BLUE)),
                     (ids.footer, widget::Canvas::new().color(color::DARK_GREEN).length(100.0))])
        .set(ids.master, ui);
    let j = widget::Button::new()
        .middle_of(ids.footer)
        .color(color::GREEN)
        .wh([200.0, 100.0])
        .set(ids.button_body, ui);
    if j.was_clicked() {
        println!("kkkk");
    }
    if let &Some(_) = &app.prompt {
        let prompt_j = PromptView::new(&mut app.prompt, promptsender)
            .color(color::LIGHT_GREY)
            .wh_of(ids.master)
            .middle_of(ids.master);
        prompt_j.set(ids.promptview, ui);
    }


}
