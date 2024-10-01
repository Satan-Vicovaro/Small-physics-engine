use std::ops::{ Mul, Sub, AddAssign};

use sdl2::render::Canvas;
use std::ops::Add;

use crate::draw_handler::DrawHandler;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    x:f64,
    y:f64,
}

#[allow(dead_code)]
impl Vector2D {
    pub fn new(vector:(f64,f64)) -> Self {
        let (x,y)=vector;
        return Vector2D {
            x:x,
            y:y,
        }
    }
    pub fn from_points(start:(f64,f64), end:(f64,f64)) -> Self {
        let (s_x,s_y) = start;
        let (e_x,e_y)= end;
        return Vector2D {
            x:e_x-s_x,
            y:e_y-s_y
        }
    }
    pub fn from_unit_vector(vector:Vector2D, lenght:f64) -> Self {
        return vector.mul_by_constant(lenght);
    }
    pub fn get_vector_value(&self) -> (f64,f64) {
        return (self.x,self.y);
    }
    pub fn dot_product(&self, other:&Vector2D) -> f64 {
        let (x_2,y_2) = other.get_vector_value();

        return self.x*x_2 + self.y*y_2;
    }
    pub fn left_normal (&self) -> Vector2D {
        return Vector2D::new((-self.y,self.x));
    }
    pub fn right_normal (&self) -> Vector2D {
        return Vector2D::new((self.y,-self.x))
    }
    pub fn length(&self) -> f64 {
        let value= self.x*self.x + self.y*self.y;
        return value.sqrt();
    }
    pub fn anlge_between_vectors(&self, other:&Vector2D) -> f64 {
        let theta= self.dot_product(other) / (self.length() * other.length());
        return theta.acos();
    }
    pub fn value_of_cross_product(&self, other:&Vector2D) -> f64 {
        return self.length()*other.length()*self.anlge_between_vectors(other).sin();
    }
    pub fn create_point(&self,start:(f64,f64)) -> (f64,f64) {
        let (start_x,start_y) = start;
        return (start_x+self.x, start_y+self.y);
    }
    pub fn mul_by_constant(& self, value:f64) ->Vector2D {
        return Vector2D {
            x: self.x * value,
            y: self.y * value,
        };
    }
    pub fn opposite_vector(& self) -> Vector2D {
        return Vector2D {
            x:-self.x,
            y:-self.y,
        };
    }
    pub fn unit_vector(& self) -> Vector2D {
        let length = self.length();
        return Vector2D {
            x:self.x/length,
            y:self.y/length
        }
    }
    pub fn draw_vector(& self, start:(f64,f64),magnitude:f64,canvas: &mut Canvas<sdl2::video::Window>) {
        DrawHandler::draw_vector(self, start, magnitude, canvas)
    }
    pub fn area_between_vectors(vector_1:Vector2D,vector_2:Vector2D) -> f64{
        return 0.5*(vector_1.x * vector_2.y - vector_2.x - vector_1.y).abs();
    }
    
    pub fn lenght_of_projection(&self, projectee:Vector2D) -> f64 {
        return self.dot_product(&projectee)/self.length();
    }
}
impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector2D {
            x:self.x + rhs.x,
            y:self.y + rhs.y,
        }
    }
}
impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector2D {
            x:self.x - rhs.x,
            y:self.y - rhs.y,
        }
    }
}
impl Mul for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: Self) -> Self::Output {
        return Vector2D {
            x:self.x * rhs.x,
            y:self.y * rhs.y,
        }
    }
}
impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}