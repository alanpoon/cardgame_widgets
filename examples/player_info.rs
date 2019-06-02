#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
extern crate find_folder;
extern crate image;
pub mod support;
use conrod_core::{widget, color, Colorable, Widget, Positionable, Sizeable, Labelable};
use conrod_core::backend::glium::glium::{self, glutin, Surface};
use conrod_core::event;
use conrod_core::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::player_info::list::List;
use cardgame_widgets::custom_widget::player_info::item::IconStruct;
use cardgame_widgets::sprite::SpriteInfo;
use cardgame_widgets::text::get_font_size_hn;
use std::time::Instant;

widget_ids! {
    pub struct Ids {
         master,
         icon_vec,
         overlay_top,
         overlay_body,
         overlay_canvas,
        overlay_image,
        overlay_text,
        overlay2
    }
}
pub struct App {
    icon_vec: Vec<IconStruct>,
    overlay: bool,
    frame: u32,
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
    let rust_logo = load_image(&display, "images/rust.png");
    let button_logo = load_image(&display, "images/arrows_but.png");
    let events_loop_proxy = events_loop.create_proxy();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut last_update = std::time::Instant::now();
    let mut image_map: conrod_core::image::Map<glium::texture::Texture2d> = conrod_core::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);
    let button_logo = image_map.insert(button_logo);
    let mut old_captured_event: Option<ConrodMessage> = None;
    let mut captured_event: Option<ConrodMessage> = None;
    let sixteen_ms = std::time::Duration::from_millis(800);
    let u: usize = 2;
    let mut app = App {
        /*
        icon_vec: vec![IconStruct(Image::new(rust_logo),
                            u.to_string(),
                            "Use to buy cards".to_owned()),
                 IconStruct(Image::new(rust_logo),
                            "(70)".to_owned(),
                            "Like blackjack,you draw one more card in hope to score more points"
                                .to_owned()),
                 IconStruct(Image::new(rust_logo),
                            "(80)".to_owned(),
                            "Use to make a card wild card".to_owned()),
                 IconStruct(Image::new(rust_logo),
                            "(70)".to_owned(),
                            "Use to make a card wild card".to_owned())],
                            */
                            
        icon_vec: vec![IconStruct(Image::new(rust_logo),"(80)".to_owned(),"Ink: like Blackjack, draw one more card in hope of scoring more points. You must however use this card to spell word".to_owned()), //ink
         IconStruct(Image::new(rust_logo),"(80)".to_owned(),"Ink Remover, You may convert an inked card back to normal. You put it back into your hand".to_owned()), //inkremover
         IconStruct(Image::new(rust_logo),"(80)".to_owned(),"Coin, You may use coin to buy new cards".to_owned()), //coin
         IconStruct(Image::new(rust_logo),"(80)".to_owned(),"Literacy Award, Construct longer words, the token go to the last player who constructed the longest word".to_owned()), //literacy award
         IconStruct(Image::new(rust_logo),"(80)".to_owned(),"Prestige, End Game Victory Point".to_owned()), //prestige
         IconStruct(Image::new(rust_logo),"(80)".to_owned(),"Size of Draft pile".to_owned()), //draftlen
         
    ],
        frame:0,
        
        overlay: false,
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
        app.frame += 1;
    }
}

fn set_widgets(ui: &mut conrod_core::UiCell,
               ids: &mut Ids,
               _app: &mut App,
               rust_logo: conrod_core::image::Id,
               button: conrod_core::image::Id) {
    widget::Canvas::new().color(color::GREEN).set(ids.master, ui);
    let w = ui.w_of(ids.master).unwrap();
    let default_color = color::GREY;
    if _app.frame >= 10 {
        widget::Canvas::new()
            .flow_down(&[(ids.overlay_top,
                          widget::Canvas::new().color(color::LIGHT_BLUE).length(100.0)),
                         (ids.overlay_body, widget::Canvas::new().color(color::YELLOW))])
            .middle_of(ids.master)
            .wh([400.0, 400.0])
            .color(color::TRANSPARENT)
            .set(ids.overlay_canvas, ui);
        let slist = List::new(_app.icon_vec.clone(), &mut _app.overlay)
            .color(default_color)
            .label("Player Info")
            .label_color(default_color.plain_contrast())
            .mid_left_of(ids.overlay_top)
            .wh_of(ids.overlay_top)
            .set(ids.icon_vec, ui);
        if let (Some(_s), Some(_si), Some(xy)) = slist {

            if let Some(&IconStruct(ref _image, _, ref _desc)) = _app.icon_vec.get(_s) {
                let _dim = [300.0, 100.0];
                widget::Canvas::new()
                    .wh(_dim)
                    .x(xy[0])
                    .color(default_color)
                    .down_from(ids.overlay_top, 0.0)
                    .set(ids.overlay2, ui);
                _image.wh([20.0, 20.0]).mid_left_of(ids.overlay2).set(ids.overlay_image, ui);
                let fontsize = get_font_size_hn(_dim[1], 4.0);
                widget::Text::new(&_desc)
                    .font_size(fontsize)
                    .color(default_color.plain_contrast())
                    .align_middle_y_of(ids.overlay_image)
                    .right_from(ids.overlay_image, 0.0)
                    .w(_dim[0] - 20.0)
                    .h_of(ids.overlay2)
                    .set(ids.overlay_text, ui);
            }
        }
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
