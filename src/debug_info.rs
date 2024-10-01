use crate::{draw_handler::DrawHandler, Vector2D};
use sdl2::render::Canvas;
use crate::movement::Movement;
use crate::{draw_handler, Shape};

pub struct DebugInfo {
    elements: Vec<DebugType>
}
pub enum DebugType {
    VectorDebug { vector:Vector2D,start:(f64,f64) },
    PointDebug { point:(f64,f64) }
}

#[allow(dead_code)]
impl DebugInfo {
    pub fn new() -> Self {
        return DebugInfo{
            elements:Vec::new(),
        }
    }
    pub fn add_element(&mut self,element:DebugType ) {
        self.elements.push(element);
    }
    pub fn reset_vector(&mut self) {
        unsafe{
            self.elements.set_len(0);
        }
    }
    pub fn draw_debug_data(&self, canvas: &mut Canvas<sdl2::video::Window>, draw_handler:&mut DrawHandler) {
        for element in self.elements.iter() {
            match element {
                DebugType::VectorDebug { vector, start } => {
                    DrawHandler::draw_vector(vector, *start, 1.0, canvas);
                }
                DebugType::PointDebug { point } => {
                    match draw_handler.draw_point(*point, canvas) {
                        Ok(_) => {},
                        Err(s) => {println!("{}",s)},
                    }
                }
            }
        }
    }
    pub fn add_data(&mut self,data:Option<(&mut Movement, &mut Shape)>) {
        match data {
            Some((movement, shape)) => {
                let center = shape.get_center_point();
                for data in movement.get_debug_data() {
                    self.add_element(DebugType::VectorDebug { vector: data, start: center });
                }
                for data in shape.get_points().iter() {
                    self.add_element(DebugType::PointDebug { point: *data });
                }
            },
            None => {},
        }
    }
}