extern crate sdl2;

use draw_handler::DrawHandler;
use game_object::GameObjectList;
use main_floor::MainFloor;
use player::Player;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use shape::Shape;
use vector_2d::Vector2D;
use std::time::Duration;
use sdl2::rect::Rect;
use std::cell::{RefCell, RefMut};
use key_pressed_and_options::KeyPressedAndOptions;

//------
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use fontdue::Font;
use fontdue_sdl2::FontTexture;
//------

const SCREEN_WIDTH:u32=1200;
const SCREEN_HEIGHT:u32=800;

mod shape;
mod game_time;
mod vector_2d;
mod draw_handler;
mod game_object;
mod movement;
mod player;
mod main_floor;
mod key_pressed_and_options;
mod debug_info;
mod collision_handler;


fn handle_events(event_pump:&mut EventPump,key_pressed: &mut KeyPressedAndOptions) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                key_pressed.quit= true;
            },
            Event::KeyDown { keycode:Some(Keycode::LCTRL),.. } => {
                key_pressed.L_shift_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::LCTRL),.. } => {
                key_pressed.L_shift_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::D),.. } => {
                key_pressed.D_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::D),.. } => {
                key_pressed.D_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::A),.. } => {
                key_pressed.A_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::A),.. } => {
                key_pressed.A_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::W),.. } => {
                key_pressed.W_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::W),.. } => {
                key_pressed.W_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::S),.. } => {
                key_pressed.S_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::S),.. } => {
                key_pressed.S_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::Q),.. } => {
                key_pressed.Q_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::Q),.. } => {
                key_pressed.Q_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::E),.. } => {
                key_pressed.E_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::E),.. } => {
                key_pressed.E_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::F3),.. } => {
                key_pressed.F3_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::F3),.. } => {
                key_pressed.F3_down = false;
            },
            Event::KeyDown { keycode:Some(Keycode::F2),.. } => {
                key_pressed.F2_down = true;
            },
            Event::KeyUp { keycode:Some(Keycode::F2),.. } => {
                key_pressed.F2_down = false;
            },
            _ => {}
        }
    }

    if key_pressed.F3_down {
        key_pressed.debug_enabled = true;
    }
    if key_pressed.F2_down {
        key_pressed.debug_enabled = false;
    }
}



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut time_info = game_time::GameTime::new(&sdl_context);
    
    let window = video_subsystem.window("Mój Fizyk Endżin", SCREEN_WIDTH, SCREEN_HEIGHT )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut keys_pressed= KeyPressedAndOptions::new();

    let player = Player::new(time_info.get_delta_t_val());
    let main_floor= MainFloor::new(time_info.get_delta_t_val());
    let mut game_objects = GameObjectList::new();

    game_objects.objects.push_back(Box::new(player));
    game_objects.objects.push_back(Box::new(main_floor));


    //----
    let texture_creator = canvas.texture_creator();
    //fontdue-sdl2:
    let mut font_texture = FontTexture::new(&texture_creator).unwrap();
    // fontdue:
    let font = include_bytes!("fonts\\ComicSans.ttf") as &[u8];
    let comic_sans = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    let font = include_bytes!("fonts\\Roboto-Bold.ttf") as &[u8];
    let roboto_regular = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let fonts = &[comic_sans,roboto_regular];
    let color = Color::RGB(0xFF, 0xFF, 0xFF);
    let layout = Layout::new(CoordinateSystem::PositiveYDown);
    font_texture.draw_text(&mut canvas, fonts, layout.glyphs()).unwrap();
    //-----
    
    let mut draw_handler = DrawHandler::new(fonts, color, layout, font_texture);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        
        handle_events(&mut event_pump,&mut keys_pressed);

        if keys_pressed.quit {
            break 'running;
        }
        // The rest of the game loop goes here...
        game_objects.handle_objects(&mut canvas, &keys_pressed, &time_info, &mut draw_handler);

        time_info.count_time();
        time_info.display_info(&mut canvas, &mut draw_handler).unwrap();
        canvas.present();
    }
}