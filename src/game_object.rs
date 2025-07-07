use core::num;
use std::collections::{hash_map, HashMap, LinkedList};
use array2d::Array2D;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use crate::collision_handler::CollisionHandler;
use crate::draw_handler::DrawHandler;
use crate::game_time::GameTime;
use crate::vector_2d::Vector2D;
use crate::{draw_handler, shape, KeyPressedAndOptions, SCREEN_HEIGHT};
use crate::shape::Shape;
use crate::movement::Movement;
use crate::debug_info::DebugInfo;


const num_of_rows_aabb:usize=10;
const num_of_cols_aabb:usize=10;
pub struct GameObjectList {
    objects:Vec<(u32,Box<dyn GameObject>)>,
    aabb:array2d::Array2D<Vec<u32>>,
    next_id_number:u32,
}
impl GameObjectList {
    pub fn new() -> Self {
        let array=Vec::with_capacity(6); 
        return GameObjectList {
            objects:Vec::new(),
            aabb:Array2D::filled_with(array, num_of_rows_aabb, num_of_rows_aabb),
            next_id_number:0,
        }
    }
    pub fn push_object (&mut self, object:Box<dyn GameObject>) {
        self.objects.push((self.next_id_number,object));
        self.next_id_number +=1;

    }
    pub fn print_aabb (&mut self) {
        for row in 0..num_of_rows_aabb {
            for col in 0..num_of_cols_aabb {
                let a = &self.aabb[(row,col)];
                for number in a.iter() {
                    print!("{} ", number);
                }
                print!("\n");
            }
        } 
    }
    fn reset_aabb(&mut self) {
        for row in 0..num_of_rows_aabb {
            for col in 0..num_of_cols_aabb {
                unsafe {
                    self.aabb[(row,col)].set_len(0);
                }
            }
        } 
    }

    fn get_mut_object(&mut self, id:u32) -> Option< &mut Box<dyn GameObject> >{
        for object in self.objects.iter_mut() {
            if object.0 == id {
                return Some(&mut object.1);
            }
        }
        return None
    }
    pub fn handle_objects(&mut self, canvas: &mut Canvas<sdl2::video::Window>,
                          keys_pressed:& KeyPressedAndOptions, time_info:& GameTime, draw_handler:&mut DrawHandler) {

        
        // for (id,object) in self.objects.iter_mut() {
        //     match object.impl_shape() {
        //         Some(shape) => {
        //             for (x,y) in shape.get_points().iter() {
        //                     let x:usize = *x as usize / 100;
        //                     let y:usize = *y as usize / 100;
        //                     if x < num_of_rows_aabb && y < num_of_cols_aabb {
        //                         self.aabb[(x,y)].push(*id);
        //                     }
        //             }
        //         },
        //         None => {},
        //     }
        // }

        // let mut offset = 0.0;
        // for row in 0..num_of_rows_aabb {
        //     for col in 0..num_of_cols_aabb {

        //         let mut possible_objects_collision:HashMap<u32, u32> = HashMap::new();
        //         for id_s in self.aabb[(row,col)].iter() {
        //              possible_objects_collision.insert(*id_s, *id_s);
        //         }
        //         if possible_objects_collision.len() >= 2 {
        //             let text = format!("possible collsion on {0},{1}",row,col );
        //             draw_handler.draw_text(&text, (600.0,50.0 + offset), canvas).unwrap();
        //             offset += 30.0;
        //         }
        //         possible_objects_collision.shrink_to(0);
        //     }
        // } 

        for (_id,object) in self.objects.iter_mut() {

            for _ in 0..time_info.get_phisic_ticks(keys_pressed) {
                object.impl_key_movement_handler(keys_pressed);

                match object.impl_movement() {
                    Some((movement, shape)) => {
                        movement.apply_movement(shape);
                        movement.apply_drag(shape);
                        movement.apply_gravity(&shape,true);
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

        //creating clone array
        let mut shapes_b:Vec<(u32,Shape)> = Vec::new();

        for i in 0..self.objects.len() {
            let x = self.objects[i].1.impl_shape().unwrap().clone();
            let id = self.objects[i].0;
            shapes_b.push((id,x));
        }
        let shapes_a = shapes_b.clone();

        let mut collision_data: Vec<((u32,u32), Vec<((f64, f64), Vector2D)>)> = Vec::new();
        // checking collisions
        for (id_a, shape_a) in shapes_a.iter()  {
            for (id_b, shape_b) in shapes_b.iter() {
                if *id_a  == *id_b {
                    //we don't check collision with itself
                    break;
                }       
                let collision: Option<Vec<((f64, f64), Vector2D)>> =
                    CollisionHandler::is_collision(&shape_b, &shape_a);
                if collision.is_some() {
                    collision_data.push(((*id_b,*id_a),collision.unwrap()));
                }
                let collision =
                     CollisionHandler::is_collision(&shape_a, &shape_b);

                if collision.is_some() {
                    collision_data.push(((*id_a,*id_b),collision.unwrap()));
                }  
            }
        }

        for (id,force_points) in collision_data.iter() {

            // object we put force on id.0 this is the edge where collision was 
        {
            let mut object_a = self.get_mut_object(id.0); 
            if object_a.is_none() {
                continue; 
            }            
            if object_a.as_mut().unwrap().impl_movement().is_none() {
                continue; // obj dont impl shapes
            }
            
            let (obj_a_movement, obj_a_shape) = object_a.as_mut().unwrap().impl_movement().unwrap();
            
            for (point, vector) in force_points.iter() {
                //obj_a_movement.apply_momentum(&obj_a_shape, vector.opposite_vector().mul_by_constant(1.0), *point);
                //obj_a_movement.apply_drag(obj_a_shape);
                obj_a_movement.apply_force(&obj_a_shape, vector.opposite_vector().mul_by_constant(1.0), *point);
            }
        }
            let mut object_b = self.get_mut_object(id.1); 
            if object_b.is_none() {
                continue; 
            }            
            if object_b.as_mut().unwrap().impl_movement().is_none() {
                continue; // obj dont impl shapes
            }
            let (obj_b_movement, obj_b_shape) = object_b.as_mut().unwrap().impl_movement().unwrap();
            for (point, vector) in force_points.iter() {
                //obj_b_movement.apply_momentum(&obj_b_shape, vector.mul_by_constant(1.0), *point);
                //obj_b_movement.apply_drag(obj_b_shape);
                obj_b_movement.apply_force(&obj_b_shape, vector.mul_by_constant(1.0), *point);
            }

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