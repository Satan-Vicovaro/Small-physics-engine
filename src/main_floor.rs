use crate::game_object::GameObject;
use crate::movement::Movement;
use crate::shape::Shape;
use crate::debug_info::DebugInfo;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use crate::KeyPressedAndOptions;
pub struct MainFloor {
    shape:Shape,
    movement:Movement,
    debug_info:DebugInfo
}
impl MainFloor {
    pub fn new(delta_t:f64) -> Self {
        let mut movement = Movement::new(delta_t);
        movement.add_angular_velocity(0.4);
        return MainFloor {
            shape:Shape::create_rect((300.0,300.0),
            0.35* SCREEN_HEIGHT as f64 ,0.35 * SCREEN_WIDTH as f64, 0.0, 1.0
                                    ),
            movement:movement,
            debug_info:DebugInfo::new()
        }
    }
}
impl GameObject for MainFloor {
    fn impl_movement(&mut self) -> Option<(&mut Movement,&mut Shape)> {
        return Some((&mut self.movement, &mut self.shape));
    }
    fn impl_shape(&mut self) -> Option<&mut Shape> {
        return Some(&mut self.shape);
    }
    fn impl_key_movement_handler(&mut self, keys_pressed:&KeyPressedAndOptions) {
         
    }
    fn impl_debug_info(&mut self) -> Option<(&mut DebugInfo,&mut Movement, &mut Shape)> {
        return Some((&mut self.debug_info,&mut self.movement, &mut self.shape));
    }
}