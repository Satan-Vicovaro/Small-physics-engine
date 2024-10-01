use std::io::copy;
use std::ops::Mul;

use fontdue::layout::{self, Layout};
use fontdue_sdl2::FontTexture;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use crate::shape::Shape;
use crate::vector_2d::Vector2D;
use sdl2::pixels::Color;
use fontdue::layout::TextStyle;

use fontdue::Font;
pub struct DrawHandler <'a> {
    fonts:&'a[Font;2] ,
    color:Color,
    layout:Layout<Color>,
    font_texture:FontTexture<'a>,
}
impl<'a>  DrawHandler <'a> {
    pub fn new(fonts:&'a[Font;2],color:Color,layout:Layout<Color>, font_texture:FontTexture <'a>) -> Self {
        return DrawHandler {
            fonts:fonts,
            color:color,
            layout:layout,
            font_texture:font_texture,
        };
    }

    pub fn draw_text(&mut self, text:&str,position:(f32,f32) ,canvas:&mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (pos_x,pos_y) = position;
        
        let mut settings = self.layout.settings().clone();
        settings.x = pos_x;
        settings.y = pos_y;
        self.layout.reset(&settings);
        
        self.layout.append(self.fonts, &TextStyle::with_user_data(&text, 20.0, 1, self.color));
        return self.font_texture.draw_text(canvas, self.fonts, self.layout.glyphs());
    }

    pub fn draw_point(&mut self,position:(f64,f64) ,canvas:&mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (pos_x,pos_y) = position ;
        let point_points = [
            (pos_x,pos_y),
            (pos_x - 1.0 ,pos_y),
            (pos_x + 1.0,pos_y),
            (pos_x,pos_y - 1.0),
            (pos_x,pos_y + 1.0),
            (pos_x - 1.0 ,pos_y - 1.0),
            (pos_x + 1.0,pos_y + 1.0),
            (pos_x - 1.0,pos_y + 1.0),
            (pos_x + 1.0,pos_y - 1.0),
            (pos_x - 2.0 ,pos_y),
            (pos_x + 2.0,pos_y),
            (pos_x,pos_y - 2.0),
            (pos_x,pos_y + 2.0),
        ];
        for (p_x, p_y) in point_points.iter() {
            canvas.draw_fpoint((*p_x as f32, *p_y as f32)).unwrap();
        }
        let text = format!("({:.1},{:.1})", pos_x,pos_y);
        
        let mut settings = self.layout.settings().clone();
        settings.x = pos_x as f32;
        settings.y = pos_y as f32 + 15.0 ;
        self.layout.reset(&settings);
        
        self.layout.append(self.fonts, &TextStyle::with_user_data(&text, 10.0, 1, self.color));
        return self.font_texture.draw_text(canvas, self.fonts, self.layout.glyphs());
    }

    pub fn draw_line_f64(start:(f64,f64),end:(f64,f64),canvas:&mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (start_x,start_y) = start;
        let (end_x,end_y) = end;

        let start= Point::new(start_x as i32, start_y as i32);
        let end= Point::new(end_x as i32,end_y as i32);

        return canvas.draw_line(start, end);
    }
    pub fn draw_vector(vector:& Vector2D, start:(f64,f64), magnitude:f64, canvas:&mut Canvas<sdl2::video::Window>) {
        if vector.get_vector_value() == (0.0,0.0) {
            return;
        }
        
        let (end_x, end_y)= vector.mul_by_constant(magnitude).create_point(start);
        let vector=vector.mul_by_constant(magnitude);
        
        DrawHandler::draw_line_f64(start, (end_x,end_y), canvas).unwrap();

        //vector oposite, with lengh of 10 units, its the size of tip of vector
        let opossite_vector = vector.unit_vector().mul_by_constant(-10.0);

        let support_point = opossite_vector.create_point((end_x,end_y));
        let left_point = opossite_vector.left_normal().create_point(support_point);
        let right_point = opossite_vector.right_normal().create_point(support_point);

        DrawHandler::draw_line_f64(left_point, (end_x,end_y), canvas).unwrap();
        DrawHandler::draw_line_f64(right_point, (end_x,end_y), canvas).unwrap();
    }

    pub fn draw_shape(shape:&mut Shape, canvas: &mut Canvas<sdl2::video::Window>) {

        let poligon_points = shape.get_points();
        for i in 0..poligon_points.len() - 1 {
            match DrawHandler::draw_line_f64(poligon_points[i], poligon_points[i+1], canvas){
                Ok(_) => {},
                Err(m) => {println!("{}",m)},
            }
        }
        match DrawHandler::draw_line_f64(poligon_points[0], poligon_points[poligon_points.len() - 1], canvas){
            Ok(_) => {},
            Err(m) => {println!("{}",m)},
        }
    }
    pub fn rotate_point(point:(f64,f64), pivot:(f64, f64), angle:f64) -> (f64,f64){
        let (mut x,mut y) = point;
        let cos_a=angle.cos();
        let sin_a=angle.sin();
        let (pivot_x, pivot_y) = pivot;

        x -= pivot_x;
        y -= pivot_y;

        let mut return_x = x*cos_a - y*sin_a ;
        let mut return_y = x*sin_a + y*cos_a ;

        return_x += pivot_x;
        return_y += pivot_y;

        return (return_x,return_y);
    }
    
}