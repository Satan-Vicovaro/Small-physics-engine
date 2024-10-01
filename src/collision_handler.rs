use sdl2::rect::Point;

use crate::shape::Shape;
use crate::Vector2D;
pub struct CollisionHandler {
    collision:bool,
}

#[allow(non_snake_case)]
impl CollisionHandler {

    pub fn is_collision(A:&Shape, B:&Shape) -> Option<Vec<(f64,f64)>> {
        let mut shape_vectors_A:Vec<Vector2D> = Vec::with_capacity(A.get_points().len());

        let points_A = A.get_points();
        let points_B = B.get_points();
        
        //creating from points_A vectors coresponding to their vertexes:
        // A----B into vector: a->b
        let last_index = A.get_points().len() - 1; 
        for index in 0..last_index {
            let vector = Vector2D::from_points(points_A[index], points_A[index + 1]);
            shape_vectors_A.push(vector);
        } 
        shape_vectors_A.push(Vector2D::from_points(points_A[last_index], points_A[0])); 

        let mut return_points:Vec<(f64,f64)> = Vec::with_capacity(3);
        for point_B in points_B.iter() {
            let mut allign_counter = 0;
            //for every point_B we check all posible seperation axis
            for (index, point_A) in points_A.iter().enumerate() {
                let separating_axis = shape_vectors_A[index];

                let projection_B_len = separating_axis.
                    lenght_of_projection(Vector2D::from_points(*point_A,*point_B));
                
                if projection_B_len > 0.0 && separating_axis.length() > projection_B_len {
                    allign_counter += 1;
                }
                else {
                    //seperation axis found
                    break;
                }
            }
            if allign_counter == shape_vectors_A.len() {
                //point is alligned wtih all walls, so there is collision
                return_points.push(*point_B);
            }
        }
        if return_points.len() == 0 {
            return None;
        }
        else {
            return Some(return_points);
        }
    }
}