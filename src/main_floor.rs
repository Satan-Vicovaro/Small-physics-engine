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
    pub fn new(delta_t:f64, postion: (f64,f64), horizontal: bool) -> Self {
        let mut shape = Shape::create_rect(
            (0.0, 0.0), 0.0, 0.0, 0.0, 0.0);
        if horizontal {
            let mut shape_1 =Shape::create_rect(
                postion,
                0.05* SCREEN_HEIGHT as f64 ,
                1.0 * SCREEN_WIDTH as f64,
                0.0,
                1.0);
            shape_1.set_immovable_object();
            shape.from_shape(&shape_1);
        }
        else {
            let mut shape_1 =Shape::create_rect(
                postion,
                1.0 * SCREEN_WIDTH as f64,
                0.05* SCREEN_HEIGHT as f64 ,
                0.0,
                1.0);
            shape_1.set_immovable_object();

            shape.from_shape(&shape_1);
        }
        
        shape.set_immovable_object();
        return MainFloor {
            shape:shape,
            movement:Movement::new(delta_t),
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