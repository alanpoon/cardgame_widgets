#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable, Labelable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;

use cardgame_widgets::custom_widget::table_list::{TableList, TableListTexts};
use cardgame_widgets::custom_widget::pad_text_button::Button;
use std::time::Instant;

widget_ids! {
    pub struct Ids {
         master,
         tablelist,
    }
}
pub struct Local {
    ready: &'static str,
    leave: &'static str,
    join: &'static str,
    playergame: &'static str,
    changeto: &'static str,
}
pub struct AppData {
    texts: Local,
}

pub struct TableListTex<'a> {
    appdata: &'a AppData,
}
impl<'a> TableListTexts for TableListTex<'a> {
    fn text_ready(&self) -> &'static str {
        self.appdata.texts.ready
    }
    fn text_leave(&self) -> &'static str {
        self.appdata.texts.leave
    }
    fn text_join(&self) -> &'static str {
        self.appdata.texts.join
    }
    fn text_playergame(&self) -> &'static str {
        self.appdata.texts.playergame
    }
    fn text_changeto(&self) -> &'static str {
        self.appdata.texts.changeto
    }
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
    ui.fonts.insert(support::assets::load_font("fonts/NotoSans/NotoSans-Regular.ttf"));
    let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut demo_text_edit = "Click here !".to_owned();
    let mut last_update = std::time::Instant::now();
    let mut c = 0;
    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(100);
    let mut app = AppData {
        texts: Local {
            ready: "ready",
            leave: "leave",
            join: "join",
            playergame: "playergame",
            changeto: "changeto",
        },
    };
    let table = vec!["alan".to_owned()];
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
                set_widgets(&mut ui, &mut ids, &app, &table);

            }
            Some(ConrodMessage::Thread(t)) => {
                let mut ui = ui.set_widgets();
                set_widgets(&mut ui, &mut ids, &app, &table);
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

fn set_widgets(ui: &mut conrod::UiCell, ids: &mut Ids, app: &AppData, table: &Vec<String>) {
    widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
    let k = TableListTex { appdata: app };
    let j = TableList::new(&k,
                                                   //ready
                                                   Box::new(|| {
                  println!("ready");
                }),
                                                   //join
                                                   Box::new(|| {
                  
                }),
                                                   //leave
                                                   Box::new(|| {
                    
                }),
                                                   //change_player_number
                                                   Box::new(|x| {
                   
                }),
                                                   table,//players
                                                   3,//table_space
                                                   4,//max_space
                                                   true//joined
                                                   )
            .w_h(1000.0, 200.0)
            .middle()
            .label_color(color::GREEN);
    j.set(ids.tablelist, ui);
}
