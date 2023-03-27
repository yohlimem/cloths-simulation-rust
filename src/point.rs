use std::ops::Mul;

use nannou::prelude::*;

// use crate::GRAVITY;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub velocity: Vec2,
    /// acceleration
    pub force: Vec2,
    pub is_kinematic: bool,
    pub id:u32,
}

impl Point {
    // pub fn addForce(&mut self, force: Vec2){
    //     self.force += force;
    // }
         
    pub fn update(&mut self, dt: f64){
        if self.is_kinematic || self.force.is_nan() {return}
        // self.air_drag(dt);
        // self.gravity(dt);
        self.pos += self.velocity.mul(dt as f32) + self.force.mul(dt as f32).mul(dt as f32); 
    }
    // pub fn new() -> Point{
    //     Point {
    //         pos: vec2(0.0, 0.0),
    //         velocity: vec2(0.0, 0.0),
    //         // aka acceleration
    //         force: vec2(0.0, 0.0),
    //         is_kinematic: false,
    //         id: 0,
    //     }
    // }
    // pub fn from(pos: Vec2, velocity: Vec2,force: Vec2,is_kinematic: bool, id: u32) -> Point{
    //     Point {
    //         pos,
    //         velocity,
    //         force,
    //         is_kinematic,
    //         id,
    //     }
    // }
    pub fn air_drag(&mut self){
        self.force = self.force.mul(0.99);
        self.velocity = self.velocity.mul(0.99);
    }
    pub fn gravity(&mut self, dt: f64){
        self.velocity.y += -4.0 * dt as f32;
    }
}