use std::ops::Mul;

use nannou::{prelude::*};
use crate::point::Point;

pub struct Spring{
    pub point1: Point,
    pub point2: Point,
    pub stiffness: f32,
    pub distance: f32,
    pub rest_length: f32,
}

impl Spring{
    pub fn update(&mut self){
        let dir:Vec2 = self.point1.pos - self.point2.pos;
        self.distance = dir.length() - self.rest_length;
        let force = dir.normalize().mul(self.distance * -self.stiffness);
        println!("force: {}, dir: {}, length {}, point1: {:?}, point2: {:?}", force, dir, self.distance, self.point1.pos, self.point2.pos);


        self.point1.force += force;
        self.point2.force += -force;

        // gravity
        self.point1.velocity.y += -8.9;
        self.point2.velocity.y += -8.9;
    }
}