use crate::{shape::Shape, vector_2d::Vector2D};

pub struct Movement {
    linear_acceleration:Vector2D,
    linear_velocity:Vector2D,
    angular_acceleration:f64,
    angular_velocity:f64,
    delta_t:f64
}

#[allow(dead_code)]
impl Movement {
    pub fn new(delta_t:f64) -> Self {
        return Movement {
            linear_acceleration:Vector2D::new((0.0,0.0)),
            linear_velocity:Vector2D::new((0.0,0.0)),
            angular_acceleration:0.0,
            angular_velocity:0.0,
            delta_t:delta_t,
        }
    }
    pub fn set_linear_acceleration(&mut self,value:Vector2D) {
        self.linear_acceleration = value;
    }
    pub fn add_linear_acceleration(&mut self, value:Vector2D) {
        self.linear_acceleration += value;
    }
    pub fn set_linear_velocity(&mut self,value:Vector2D) {
        self.linear_velocity = value;
    }
    pub fn add_linear_velocity(&mut self,value:Vector2D) {
        self.linear_velocity += value;
    }
    pub fn set_angular_acceleration(&mut self,value:f64) {
        self.angular_acceleration = value;
    }
    pub fn add_angular_acceleration(&mut self,value:f64) {
        self.angular_acceleration += value;
    }
    pub fn set_angular_velocity(&mut self,value:f64) {
        self.angular_velocity = value;
    }
    pub fn add_angular_velocity(&mut self,value:f64) {
        self.angular_velocity += value;
    }
    pub fn apply_movement(&mut self, shape:&mut Shape) {

        self.linear_velocity += self.linear_acceleration.mul_by_constant(self.delta_t);

        let delta_s = self.linear_velocity.mul_by_constant(self.delta_t);     
        shape.shift_shape(delta_s);

        self.angular_velocity += self.angular_acceleration*self.delta_t;
        let angle = self.angular_velocity*self.delta_t;
        shape.add_to_angle(angle);
    }
    pub fn get_debug_data(&self) -> [Vector2D;2] {
        return [
            self.linear_acceleration,
            self.linear_velocity,
        ];
    }
}