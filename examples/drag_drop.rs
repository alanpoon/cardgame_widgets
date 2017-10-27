#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;

use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable};
use conrod::backend::glium::glium::{self, glutin, Surface};
use conrod::event;
use conrod::widget::envelope_editor::EnvelopePoint;
use cardgame_widgets::custom_widget::dragdrop_list;
use std::time::Instant;

widget_ids! {
    pub struct Ids {
         master,
         wraplist,
    }
}
pub struct App {
    dragging: bool,
    hash: [Option<(conrod::position::Point, color::Color)>; 25],
    temp:[Option<(conrod::position::Point, color::Color)>; 25],
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
    color_hash[0] = Some(([0.0, 0.0], color::BLACK));
    color_hash[1] = Some(([0.0, 0.0], color::YELLOW));
    let mut app = App {
        hash: color_hash.clone(),
        temp:color_hash,
        dragging: false,
    };
    println!("app {:?}", app.hash.clone());
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
    let (mut item0, mut dragitem0) =
        dragdrop_list::DragDropList::new(2).middle_of(ids.master).set(ids.wraplist, ui);
    let app_pos = app.hash.clone();
    let mut k_h_iter = app_pos.iter();
    let mut c = 0;
    let mut populated_j_ids =vec![];
    while let (Some(item), Some(&Some(k_h))) =
        (item0.next(ui),  k_h_iter.next()) {
        let j = widget::bordered_rectangle::BorderedRectangle::new([50.0, 50.0])
            .color(k_h.clone().1);
        let j_id = item.set(j.clone(), 50.0, ui);
        app.hash[c] = Some((ui.xy_of(j_id).unwrap(), k_h.clone().1));
        app.temp[c] = Some((ui.xy_of(j_id).unwrap(), k_h.clone().1));
        populated_j_ids.push(j_id);
        c += 1;
    }
    let mut k_h_iter2 = app_pos.iter();
    let mut populated_j_ids_iter = populated_j_ids.iter();
    let mut c2=0;
    while let ( Some(dragitem),Some(&_j_id),Some(&Some(k_h))) = (dragitem0.next(ui),populated_j_ids_iter.next(), k_h_iter2.next()){
            let j = widget::bordered_rectangle::BorderedRectangle::new([50.0, 50.0])
            .color(k_h.clone().1);
         let mut mouse_point = None;
         let rect_dim = ui.wh_of(_j_id).unwrap();
        if let Some(mouse) = ui.widget_input(_j_id).mouse() {
            if mouse.buttons.left().is_down() {
                mouse_point = Some(mouse.abs_xy());
            } else if mouse.buttons.left().is_up() {
                println!("mouse up");
                mouse_point = None;
            }
        }
        if let Some(m_point) = mouse_point {
            dragitem.set(j, m_point, ui);
            let mut _c = 0;
            for _j in app.hash.iter() {
                if let &Some(_p) = _j {
                    println!("m_point {:?},_p.0 {:?}, 0.5rect {:?}",m_point,_p.0,rect_dim[0] * 0.5);
                    if (m_point.get_x() >= _p.0.get_x() - rect_dim[0] ) & //-491 >= -487-25
                       (m_point.get_x() <= _p.0.get_x() + rect_dim[0] ) &//-491 <=-487+25
                       (m_point.get_y() >= _p.0.get_y() - rect_dim[1] ) & //296.9 >= 327-25
                       (m_point.get_y() <= _p.0.get_y() + rect_dim[1] ) { //296.9 <=327+25
                        break;
                    }
                    _c += 1;
                }

            }
            rearrange(c2, _c, &mut app.temp);
        } else {
            let temp_c = app.temp.clone();
            app.hash = temp_c;
        }
        c2+=1;
    }

}

//conrod::position::Point
fn rearrange<T: Clone>(selected_i: usize,
                       corrected_i: usize,
                       hash: &mut [Option<([f64; 2], T)>; 25]) {
                           println!("select{}, corrected{}",selected_i,corrected_i);
    let hash_c = hash.clone();
    for _i in 0..hash.len() {
        if _i == corrected_i {
            hash[_i] = match (&hash_c[selected_i],&hash_c[_i])  {
                (&Some((_,ref a_s)),&Some((pos,_)))=>Some((pos,a_s.clone())),
                _=>None
            };
        }
        if selected_i < corrected_i {
            //moved backward ____S__->__C
            if (_i < corrected_i) & (_i >= selected_i) {
                println!("move backward");
                // ____S~~~~C;
                hash[_i] = match (&hash_c[_i + 1],&hash_c[_i])  {
                (&Some((_,ref a_s)),&Some((pos,_)))=>Some((pos,a_s.clone())),
                _=>None
            };
            }
        } else if selected_i > corrected_i {
            //moved foward _____C__<-S
            if (_i <= selected_i) & (_i > corrected_i) {
                println!("move forward");
                // ____C~~~S
                hash[_i] = match (&hash_c[_i - 1],&hash_c[_i])  {
                (&Some((_,ref a_s)),&Some((pos,_)))=>Some((pos,a_s.clone())),
                _=>None
            };
            }
        }
        
    }
}
