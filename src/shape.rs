use sdl2::render::Canvas;

use crate::{draw_handler::DrawHandler, vector_2d::Vector2D};

#[derive(Debug, Clone, PartialEq)]
pub struct Shape{
    poligon_points:Vec<(f64,f64)>,
    poligon_points_no_rotation:Vec<(f64,f64)>,
    angle: f64,
    density: f64,
    inv_mass:f64,
    area:f64
}

#[allow(dead_code)]
impl Shape {
    pub fn create_rect(position:(f64,f64), height:f64, width:f64, angle:f64,density:f64) -> Self {

        let (x,y)=position;

        let poligon_points = vec![
            (x,y),
            (x,y+height),
            (x+width,y+height),
            (x+width,y)
        ];
        let poligon_points_no_rotation = poligon_points.clone();
        let area = height*width;
        let inv_mass = density/area;

        return Shape {
            poligon_points:poligon_points,
            poligon_points_no_rotation:poligon_points_no_rotation,
            angle: angle,
            density: density,
            inv_mass:inv_mass,
            area:area,
        }
    }
    
    pub fn create_poligon(poligon_points:Vec<(f64,f64)>, angle:f64,density:f64) -> Shape{
        let last_index = poligon_points.len() - 1;
        
        // calculates area, devides poligon into triangles and then adds them up
        let mut area = 0.0;
        for i in 0..poligon_points.len() - 2 {
            let vector_1 = Vector2D::from_points(poligon_points[i], poligon_points[i + 1]);
            let vector_2 = Vector2D::from_points(poligon_points[i],poligon_points[last_index]);
            area += Vector2D::area_between_vectors(vector_1, vector_2);
        }

        let inv_mass = density/area;
        let poligon_points_no_rotation = poligon_points.clone(); 
        return Shape {
            poligon_points:poligon_points,
            poligon_points_no_rotation:poligon_points_no_rotation,
            angle:angle,
            density:density,
            inv_mass:inv_mass,
            area:area,
        }
    }
    pub fn get_inv_mass(&self) -> f64 {
        return self.inv_mass;
    }
    pub fn set_immovable_object(&mut self) {
        self.inv_mass = 0.0;
    }
    pub fn get_area(&self)  -> f64{
        let last_index = self.poligon_points.len() - 1;
        let mut area = 0.0;
        
        // calculates area, devides poligon into triangles and then adds them up
        for i in 0..last_index - 1 {
            let vector_1 = Vector2D::from_points(self.poligon_points[i], self.poligon_points[i + 1]);
            let vector_2 = Vector2D::from_points(self.poligon_points[i],self.poligon_points[last_index]);
            area += Vector2D::area_between_vectors(vector_1, vector_2);
        }
        return area;
    }
    pub fn get_mut_points_no_rotation(&mut self) -> &mut Vec<(f64,f64)>{
        return &mut self.poligon_points_no_rotation;
    }
    pub fn get_points(& self) -> &Vec<(f64,f64)> {
        return &self.poligon_points;
    }
    pub fn get_center_point(&self) -> (f64,f64){
        let mut sum_x =0.0;
        let mut sum_y =0.0;
        for (x,y) in self.poligon_points_no_rotation.iter() {
            sum_x += x;
            sum_y += y;
        }
        let num_of_points=self.poligon_points.len() as f64;
        return (sum_x/num_of_points, sum_y/num_of_points)
    }
    pub fn shift_shape(&mut self, value: Vector2D) {
        let (val_x,val_y) = value.get_vector_value();
        for (point_x, point_y) in self.poligon_points_no_rotation.iter_mut() {
            *point_x += val_x;
            *point_y += val_y;
        }
        for (point_x, point_y) in self.poligon_points.iter_mut() {
            *point_x += val_x;
            *point_y += val_y;
        }
    }
    fn rotate_shape(&mut self){
        let pivot_point = self.get_center_point();
        
        for (index, no_rot_point) in self.poligon_points_no_rotation.iter().enumerate(){
            let rot_point = DrawHandler::rotate_point(*no_rot_point, pivot_point, self.angle);
            self.poligon_points[index] = rot_point;
        }
    }
    pub fn set_angle(&mut self, angle:f64) {
         self.angle = angle;
         self.rotate_shape();
    }
    pub fn add_to_angle(&mut self, angle:f64) {
        self.angle += angle; 
        self.rotate_shape();
    }
    pub fn get_angle(& self) -> f64 {
        return self.angle;
    }
    pub fn draw_shape(&mut self,canvas: &mut Canvas<sdl2::video::Window>) {
        DrawHandler::draw_shape(self, canvas)
    }
}