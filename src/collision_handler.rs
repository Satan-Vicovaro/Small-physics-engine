use sdl2::rect::Point;

use crate::shape::Shape;
use crate::movement::Movement;
use crate::{movement, Vector2D};
pub struct CollisionHandler {
    collision:bool,
}

#[allow(non_snake_case)]
impl CollisionHandler {

    pub fn manage_collision(shape_a:&Shape, shape_b:&Shape) {
        let b_in_a = CollisionHandler::is_collision(&shape_a, &shape_b);
        let a_in_b = CollisionHandler::is_collision(&shape_a, &shape_b);

        if a_in_b.is_some() && b_in_a.is_some() {

        }
        match a_in_b {
            Some(collision_points) => {

            },
            None => {},
        }
    }

    pub fn apply_collision(shape_a:&Shape, movement_a:&mut Movement,
                           shape_b:&Shape, movement_b:&mut Movement,
                           collision_points:Vec<(f64,f64)>
                        ) {
        let (mut col_point_x,mut  col_point_y) = (0.0, 0.0); 
        for (p_x,p_y) in collision_points.iter() {
            col_point_x += *p_x;
            col_point_y += *p_y;
        }
        let collision_point = (col_point_x/collision_points.len() as f64, col_point_y/collision_points.len() as f64);
        
    }

    pub fn is_collision(A:&Shape, B:&Shape) -> Option<Vec<((f64,f64),Vector2D)>> {
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

        let mut return_points:Vec<((f64,f64),Vector2D)> = Vec::with_capacity(3);
        for point_B in points_B.iter() {
            let mut allign_counter = 0;
            let mut the_closest_wall_vector:Vector2D = Vector2D::new((1000.0,1000.0));
            //for every point_B we check all posible seperation axis
            for (index, point_A) in points_A.iter().enumerate() {
                let separating_axis = shape_vectors_A[index];

                let vector_a_b = Vector2D::from_points(*point_A,*point_B);
                let projection_B_len = separating_axis.lenght_of_projection(vector_a_b);
                
                if projection_B_len > 0.0 && separating_axis.length() > projection_B_len {
                    allign_counter += 1;
                    
                    let wall_vector = Vector2D::from_points(
                        *point_B, 
                        separating_axis.projected_point(*point_A, vector_a_b));
                    if wall_vector.length() < the_closest_wall_vector.length() {
                        the_closest_wall_vector = wall_vector;
                    }
                }
                else {
                    //seperation axis found
                    break;
                }
            }
            if allign_counter == shape_vectors_A.len() {
                //point is alligned wtih all walls, so there is collision
                return_points.push((*point_B,the_closest_wall_vector));
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