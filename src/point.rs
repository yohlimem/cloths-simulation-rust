use std::ops::Mul;

use nannou::prelude::*;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub velocity: Vec2,
    /// acceleration
    pub force: Vec2,
    pub is_kinematic: bool,
}

impl Point {
    pub fn addForce(&mut self, force: Vec2){
        self.force += force;
    }
         
    pub fn update(&mut self, dt: f64){
        if self.is_kinematic {return}
        self.pos += self.velocity.mul(dt as f32) + self.force.mul(dt as f32).mul(dt as f32); 
    }
}