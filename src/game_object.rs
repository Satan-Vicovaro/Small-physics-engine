use std::collections::LinkedList;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use crate::collision_handler::CollisionHandler;
use crate::draw_handler::DrawHandler;
use crate::game_time::GameTime;
use crate::{draw_handler, shape, KeyPressedAndOptions};
use crate::shape::Shape;
use crate::movement::Movement;
use crate::debug_info::DebugInfo;

pub struct GameObjectList {
    pub objects:LinkedList<Box<dyn GameObject>>,
}
impl GameObjectList {
    pub fn new() -> Self {
        return GameObjectList {
            objects:LinkedList::new(),
        }
    }

    pub fn handle_objects(&mut self, canvas: &mut Canvas<sdl2::video::Window>,
                          keys_pressed:& KeyPressedAndOptions, time_info:& GameTime, draw_handler:&mut DrawHandler) {
        for object in self.objects.iter_mut() {
            
            for _ in 0..time_info.get_phisic_ticks() {
                object.impl_key_movement_handler(keys_pressed);

                match object.impl_movement() {
                    Some((movement, shape)) => {
                        movement.apply_movement(shape);
                    },
                    None => {},
                }
            }
            
            if keys_pressed.debug_enabled {
                match object.impl_debug_info() {
                    Some((debug_info,movement, shape)) => {
                        debug_info.add_data(Some((movement,shape)));
                        debug_info.draw_debug_data(canvas, draw_handler);
                        debug_info.reset_vector();
                    },
                    None => {},
                }
            }
            
            match object.impl_shape() {
                Some(shape) => {shape.draw_shape(canvas);},
                None => {},
            }
        }
        let a = self.objects.front_mut().unwrap();

        let shape_a = match a.impl_shape() {
            Some(shape) => {shape},
                None => {return},
        };
        let shape_a = shape_a.clone();
        
        let b = self.objects.back_mut().unwrap();
        let shape_b = match b.impl_shape() {
            Some(shape) => {shape},
                None => {return},
        };
        let shape_b = shape_b.clone();
        
        match CollisionHandler::is_collision(&shape_a, &shape_b) {
            Some(collision_points) => {
                for (number,(point_x,point_y)) in collision_points.iter().enumerate(){
                    // let text = format!("Collision at:({:.1}, {:.1}),",point_x,point_y );
                    // draw_handler.draw_text(&text, (150.0,40.0 * number as f32), canvas).unwrap();
                    draw_handler.draw_point((*point_x,*point_y), canvas).unwrap();
                }
                
            },
            None => {},
        }
        match CollisionHandler::is_collision(&shape_b, &shape_a) {
            Some(collision_points) => {
                for (number,(point_x,point_y)) in collision_points.iter().enumerate(){
                    let text = format!("Collision at:({:.1}, {:.1}),",point_x,point_y );
                    draw_handler.draw_text(&text, (150.0,40.0 * number as f32), canvas).unwrap();
                    draw_handler.draw_point((*point_x,*point_y), canvas).unwrap();
                }
                
            },
            None => {
                draw_handler.draw_text("No collision ;c", (150.0,0.0), canvas).unwrap();
            },
        }
    }
}
#[allow(dead_code)]
pub trait GameObject {
    fn impl_shape(&mut self) -> Option<&mut Shape>;
    fn impl_movement(&mut self) -> Option<(&mut Movement, &mut Shape)>;
    fn impl_key_movement_handler(&mut self, keys_pressed:&KeyPressedAndOptions);
    fn impl_debug_info(&mut self) -> Option<(&mut DebugInfo,&mut Movement, &mut Shape)>;
}