#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;

use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable, Labelable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;
use conrod::widget::envelope_editor::EnvelopePoint;
use cardgame_widgets::custom_widget::dragdrop_list;

use std::time::Instant;

widget_ids! {
    pub struct Ids {
         master,
         wraplist,
         floating_a
    }
}
pub struct App {
    last_release: Option<std::time::Instant>,
    hash: [Option<(conrod::position::Point, color::Color, Option<widget::Id>)>; 25],
    temp: [Option<(conrod::position::Point, color::Color, Option<widget::Id>)>; 25],
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
    let mut color_hash = [None; 25];
    color_hash[0] = Some(([0.0, 0.0], color::DARK_YELLOW, None));
    color_hash[1] = Some(([0.0, 0.0], color::YELLOW, None));
    color_hash[2] = Some(([0.0, 0.0], color::DARK_BLUE, None));
    color_hash[3] = Some(([0.0, 0.0], color::LIGHT_PURPLE, None));
    let mut app = App {
        hash: color_hash.clone(),
        temp: color_hash,
        last_release: Some(std::time::Instant::now()),
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
    let mut item0 = dragdrop_list::DragDropList::new(app.hash.len())
        .wh([200.0, 200.0])
        .middle_of(ids.master)
        .set(ids.wraplist, ui);
    let app_pos = app.hash.clone();
    let mut k_h_iter = app_pos.iter();
    let mut c = 0;
    let mut populated_j_ids = vec![];
    let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::WHITE);
    floating.down_from(ids.wraplist, 0.0)
        .title_bar("Blue")
        .color(color::BLUE)
        .set(ids.floating_a, ui);
    let j = widget::Canvas::new().w_h(50.0, 50.0);
    if let Some(_) = app.last_release.clone() {
        println!("reseting");
        while let (Some(item), Some(&Some(k_h))) = (item0.next(ui), k_h_iter.next()) {
            app.last_release = None;
            let k = item.set(j.color(k_h.clone().1), 50.0, ui);
            app.hash[c] = Some((ui.xy_of(k).unwrap(), k_h.clone().1, Some(k)));
            app.temp[c] = Some((ui.xy_of(k).unwrap(), k_h.clone().1, Some(k)));
            populated_j_ids.push(k);
            c += 1;
        }
    } else {
        println!("dragging");
        while let (Some(item), Some(&Some(k_h))) = (item0.next(ui), k_h_iter.next()) {
            let k = item.set(j.title_bar("Blue").color(k_h.clone().1).floating(true),
                             50.0,
                             ui);
            populated_j_ids.push(k);
            c += 1;
        }
    }
    let mut k_h_iter2 = app_pos.iter();
    let mut populated_j_ids_iter = populated_j_ids.iter();
    let mut c2 = 0;
    let mut mouse_point = None;
    let mut rect_dim = None;
    while let (Some(&_j_id), Some(&Some(k_h))) = (populated_j_ids_iter.next(), k_h_iter2.next()) {
        if let Some(mouse) = ui.widget_input(_j_id).mouse() {
            if mouse.buttons.left().is_down() {
                mouse_point = Some((c2, mouse.abs_xy()));
                rect_dim = Some(ui.wh_of(_j_id).unwrap());
            } else if mouse.buttons.left().is_up() {
                mouse_point = None;
                rect_dim = None;
            }
        }

        c2 += 1;
    }
    if let (Some((c2, m_point)), Some(rect_dim)) = (mouse_point, rect_dim) {
        let mut _c = 0;
        for _j in app.hash.iter() {
            if _c == c2 {
                _c += 1;
                continue;
            }
            if let &Some((_, _, Some(p_widget))) = _j {
                let _p_rect = ui.rect_of(p_widget).unwrap();
                if _p_rect.is_over(m_point) {
                    break;
                }
                _c += 1;
            }

        }
        let mut len_of_some = 0;
        for _i in 0..app.hash.len() {
            if let None = app.hash[_i] {
                break;
            } else {
                len_of_some += 1;
            }
        }
        let _k = if _c >= len_of_some { c2 } else { _c };
        if _k != c2 {
            rearrange(c2, _k, &mut app.temp); //(selected,new,..)
        }
    } else {
        let now = std::time::Instant::now();
        if let &None = &app.last_release {
            app.last_release = Some(now);
        }
        let temp_c = app.temp.clone();
        app.hash = temp_c;
    }
}

//conrod::position::Point
fn rearrange<T: Clone>(selected_i: usize,
                       corrected_i: usize,
                       hash: &mut [Option<([f64; 2], T, Option<widget::Id>)>; 25]) {
    let hash_c = hash.clone();
    for _i in 0..hash.len() {
        if _i == corrected_i {
            hash[_i] = match (&hash_c[selected_i], &hash_c[_i]) {
                (&Some((_, ref a_s, _)), &Some((pos, _, ref k))) => {
                    Some((pos, a_s.clone(), k.clone()))
                }
                _ => None,
            };
        }
        if selected_i < corrected_i {
            //moved backward ____S__->__C
            if (_i < corrected_i) & (_i >= selected_i) {
                //println!("move backward");
                // ____S~~~~C;
                hash[_i] = match (&hash_c[_i + 1], &hash_c[_i]) {
                    (&Some((_, ref a_s, _)), &Some((pos, _, ref k))) => {
                        Some((pos, a_s.clone(), k.clone()))
                    }
                    _ => None,
                };
            }
        } else if selected_i > corrected_i {
            //moved foward _____C__<-S
            if (_i <= selected_i) & (_i > corrected_i) {
                //println!("move forward");
                // ____C~~~S
                hash[_i] = match (&hash_c[_i - 1], &hash_c[_i]) {
                    (&Some((_, ref a_s, _)), &Some((pos, _, ref k))) => {
                        Some((pos, a_s.clone(), k.clone()))
                    }
                    _ => None,
                };
            }
        }

    }
}
