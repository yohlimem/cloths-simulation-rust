use std::ops::Mul;

use nannou::{prelude::*};
use crate::point::Point;

#[derive(Debug)]
pub struct Spring{
    pub point1: Point,
    pub point2: Point,
    pub stiffness: f32,
    pub distance: f32,
    pub rest_length: f32,
}

impl Spring{
    pub fn update(rest_length: f32, point1:Point, point2: Point) -> Vec2{
        let dir:Vec2 = (point2.pos - point1.pos).normalize();
        let delta_d = (point1.pos).distance(point2.pos) - rest_length;
        

        return delta_d * dir / 2.0;
        // println!("force: {}, dir: {}, length {}, point1: {:?}, point2: {:?}", force, dir, self.distance, self.point1.pos, self.point2.pos);
    }
}