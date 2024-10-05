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
    pub fn apply_force(&mut self,shape:& Shape, force:Vector2D, point:(f64,f64)) {
        let radius_vector = Vector2D::from_points(shape.get_center_point(), point);

        let mut angle_between_vectors = radius_vector.anlge_between_vectors(&force);
        
        angle_between_vectors+=3.14/4.0;
        let delta_v = force.unit_vector();
        let value = shape
        .get_inv_mass()*
        radius_vector.length()*
        angle_between_vectors.cos();
        
        delta_v.mul_by_constant(value);
        //self.add_linear_velocity(delta_v); 

        let delta_omega = radius_vector.determinant(&force) * shape.get_inv_mass();

        self.add_angular_velocity(delta_omega);

    }
    pub fn apply_force_with_pivot(&mut self,shape:& Shape, force:Vector2D, point:(f64,f64),pivot:(f64,f64)) {
        let radius_vector = Vector2D::from_points(pivot, point);

        let delta_omega = radius_vector.determinant(&force) * shape.get_inv_mass();
        //radius_vector.length();
        self.add_angular_velocity(delta_omega);
        todo!();

    }
}