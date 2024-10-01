use crate::{debug_info::DebugInfo, game_object:: GameObject, movement::Movement, shape::Shape, vector_2d::Vector2D};
use crate::KeyPressedAndOptions;
pub struct  Player  {
    shape:Shape,
    movement:Movement,
    debug_info:DebugInfo,
}

impl Player {
    pub fn new(delta_t:f64) -> Self {

        return Player {
            shape:Shape::create_rect((300.0,500.0), 150.0, 70.0, 0.0, 1.0),
            movement:Movement::new(delta_t),
            debug_info:DebugInfo::new(),
        }
    }
}
 impl GameObject for Player {
    fn impl_movement(&mut self) -> Option<(&mut Movement, &mut Shape)> {
        return Some((&mut self.movement,&mut self.shape));
    }
    fn impl_shape(&mut self) -> Option<&mut Shape> {
        return Some(&mut self.shape);
    }
    fn impl_key_movement_handler(&mut self, keys_pressed:&KeyPressedAndOptions) {
        let movement = &mut self.movement;
        let mut vector_val=Vector2D::new((0.0,0.0));       
        if keys_pressed.A_down {
            vector_val += Vector2D::new((-250.0,0.0));
        }
        if keys_pressed.D_down {
            vector_val += Vector2D::new((250.0,0.0));
        }
        if keys_pressed.S_down {
            vector_val += Vector2D::new((0.0,250.0));
        }
        if keys_pressed.W_down {
            vector_val += Vector2D::new((0.0,-250.0));
        }
        movement.set_linear_velocity(vector_val);

        let mut angular_velocity = 0.0;
        if keys_pressed.Q_down {
            angular_velocity -= 3.0;
        }
        if keys_pressed.E_down {
            angular_velocity += 3.0;
        }
        movement.set_angular_velocity(angular_velocity);
    }
    fn impl_debug_info(&mut self) -> Option<(&mut DebugInfo,&mut Movement, &mut Shape)> {
        return Some((&mut self.debug_info, &mut self.movement, &mut self.shape))
    }
}