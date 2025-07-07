use crate::game_object::GameObject;
use crate::movement::Movement;
use crate::Shape;
use crate::debug_info::DebugInfo;

pub struct BasicObject {
    shape:Shape,
    movement:Movement,
    debug_info:DebugInfo,
}
impl BasicObject {
    pub fn new(delta_t:f64,position:(f64,f64)) -> Self {
        let mut movement = Movement::new(delta_t);
        let shape = Shape::create_rect(position, 430.0, 150.0, 0.0, 1.0);
        movement.apply_gravity(&shape,true);
        return BasicObject {
            shape: shape,
            movement:movement,
            debug_info:DebugInfo::new(),
        }
    }
}
impl GameObject for BasicObject {
    fn impl_key_movement_handler(&mut self, keys_pressed:&crate::key_pressed_and_options::KeyPressedAndOptions) {
        
    }
    fn impl_shape(&mut self) -> Option<&mut Shape> {
        return Some(&mut self.shape);
    }
    fn impl_movement(&mut self) -> Option<(&mut Movement, &mut Shape)> {
        return Some((&mut self.movement,&mut self.shape));
    }
    fn impl_debug_info(&mut self) -> Option<(&mut DebugInfo,&mut Movement, &mut Shape)> {
        return Some((&mut self.debug_info, &mut self.movement, &mut self.shape))
    }
}