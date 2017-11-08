#[macro_use]
extern crate conrod;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable, Labelable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;
use conrod::widget::{Oval, Rectangle};
use conrod::widget::button::{Button, Flat};
use cardgame_widgets::custom_widget::instructionset::{InstructionSet, Instructable};
use std::time::Instant;
use std::sync::mpsc::Sender;
widget_ids! {
    pub struct Ids {
         master,
         body,
         footer,
         instructionset,
    }
}
pub struct App {
    instructions1: Vec<&'static str>,
    instructions2: Vec<([f64; 4], Option<[f64; 4]>)>, //([l,t,w,h_of rect],Some([l,t,w,h of oval]))
    next: &'static str,
    print_instruction: bool,
}
pub struct Instruction<'a>(&'a str, &'a [f64; 4], &'a Option<[f64; 4]>);
impl<'a> Instructable<'a> for Instruction<'a> {
    fn label(&self) -> &'a str {
        self.0
    }
    fn rect(&self, wh: [f64; 2]) -> Rectangle {
        widget::Rectangle::fill_with([self.1[2].clone() * wh[0], self.1[3].clone() * wh[1]],
                                     color::BLACK.with_alpha(0.3))
                .top_left_with_margins(self.1[0] * wh[0], self.1[1] * wh[1])

    }
    fn button(&self, wh: [f64; 2]) -> Button<Flat> {
        widget::Button::new().w_h(100.0, 50.0).mid_bottom()
    }
    fn oval_one(&self, wh: [f64; 2]) -> Option<Oval> {
        if let Some(_dim) = self.2.clone() {
            Some(widget::Oval::outline_styled([_dim[2] * wh[0], _dim[3] * wh[1]],
                                              widget::line::Style::new().thickness(5.0))
                         .top_left_with_margins(_dim[0] * wh[0], _dim[1] * wh[1]))
        } else {
            None
        }

    }
    fn oval_two(&self, wh: [f64; 2]) -> Option<Oval> {
        if let Some(_dim) = self.2.clone() {
            Some(widget::Oval::outline_styled([_dim[2] * wh[0] * 1.2, _dim[3] * wh[1]],
                                              widget::line::Style::new().thickness(5.0))
                         .top_left_with_margins(_dim[0] * wh[0], _dim[1] * wh[1]))
        } else {
            None
        }

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
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut last_update = std::time::Instant::now();
    let image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();

    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(1000);

    let mut app = App {
        instructions1: vec!["instruction 1", "instruction 2"],
        instructions2: vec![([0.4, 0.4, 0.2, 0.2], Some([0.2, 0.3, 0.1, 0.1])),
                            ([0.4, 0.4, 0.2, 0.2], None)],
        next: "next",
        print_instruction: true,
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
            let _input = match conrod::backend::winit::convert_event(event.clone(), &display) {
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
                set_widgets(&mut ui, &mut ids, &mut app);

            }
            Some(ConrodMessage::Thread(_t)) => {
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
    widget::Canvas::new()
        .color(color::BLUE)
        .flow_down(&[(ids.body, widget::Canvas::new().color(color::BLUE)),
                     (ids.footer, widget::Canvas::new().color(color::DARK_GREEN).length(100.0))])
        .set(ids.master, ui);
    let g_vec = app.instructions1
        .iter()
        .zip(app.instructions2.iter())
        .map(|(ref label, &(ref rect_tuple, ref oval_option))| {
                 Instruction(label, rect_tuple, oval_option)
             })
        .collect::<Vec<Instruction>>();
    if app.print_instruction {
        let prompt_j =
            InstructionSet::new(&g_vec, app.next).parent_id(ids.master).label_color(color::WHITE);
        app.print_instruction = prompt_j.set(ids.instructionset, ui);
    }

}
