#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod_core::{widget, color, Colorable, Widget, Positionable, Sizeable};
use conrod_core::backend::glium::glium::{self, glutin, Surface};
use conrod_core::event;
use std::time::Instant;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use cardgame_widgets::custom_widget::image_panels::{Panelable, ImagePanels, ImageRectType};
widget_ids! {
    pub struct Ids {
         master,
         panel,
    }
}
#[derive(Debug)]
pub struct Panel_Info<'a> {
    text: Option<String>,
    display_pic: Option<ImageRectType>,
    list_image: Vec<ImageRectType>,
    list_selected: &'a mut HashSet<usize, RandomState>,
}
impl<'b> Panelable for Panel_Info<'b> {
    fn text(&self) -> Option<String> {
        self.text.clone()
    }
    fn display_pic(&self) -> Option<ImageRectType> {
        self.display_pic
    }
    fn list_image(&self) -> Vec<ImageRectType> {
        self.list_image.clone()
    }
    fn list_selected<'a>(&'a self) -> &'a HashSet<usize, RandomState> {
        &self.list_selected
    }
    fn list_selected_mut<'a>(&'a mut self) -> &'a mut HashSet<usize, RandomState> {
        self.list_selected
    }
}
pub struct App {
    normal_stuff: Vec<(Option<String>, Option<ImageRectType>, Vec<ImageRectType>)>,
    list_selecteds: Vec<HashSet<usize, RandomState>>,
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

    let rust_logo = load_image(&display, "images/rust.png");
    let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut demo_text_edit = "Click here !".to_owned();
    let mut last_update = std::time::Instant::now();
    let mut c = 0;
    let mut image_map: conrod_core::image::Map<glium::texture::Texture2d> = conrod_core::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(800);
    let mut app = App {
        normal_stuff: vec![(Some("ALAN".to_string()),
                            Some((rust_logo, None)),
                            vec![(rust_logo, None), (rust_logo, None)]),
                           (Some("JAMES".to_string()),
                            Some((rust_logo, None)),
                            vec![(rust_logo, None)]),
                           (Some("MELON".to_string()),
                            Some((rust_logo, None)),
                            vec![(rust_logo, None)]),
                           (Some("OUT".to_string()),
                            Some((rust_logo, None)),
                            vec![(rust_logo, None)])],
        list_selecteds: vec![HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()],
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
            let input = match conrod_core::backend::winit::convert_event(event.clone(), &display) {
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

fn set_widgets(ui: &mut conrod_core::UiCell,
               ids: &mut Ids,
               _app: &mut App,
               rust_logo: conrod_core::image::Id) {
    widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
    let normal_stuff_c = _app.normal_stuff.clone();
    let mut vec_p = normal_stuff_c.iter()
        .zip(_app.list_selecteds.iter_mut())
        .map(|(normal, list_selected)| {
            Panel_Info {
                text: normal.0.clone(),
                display_pic: normal.1.clone(),
                list_image: normal.2.clone(),
                list_selected: list_selected,
            }
        })
        .collect::<Vec<Panel_Info>>();
    ImagePanels::new(&mut vec_p)
        .middle_of(ids.master)
        .padded_wh_of(ids.master, 20.0)
        .y_item_height(170.0)
        .x_item_list([100.0, 100.0, 22.0, 5.0])
        .set(ids.panel, ui);
    println!("vec_p {:?}", vec_p);
}
fn load_image(display: &glium::Display, path: &str) -> glium::texture::Texture2d {
    let rgba_image = support::assets::load_image(path).to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                       image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
