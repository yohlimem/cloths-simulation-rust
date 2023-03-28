use std::ops::Mul;

use nannou::prelude::*;

// use crate::GRAVITY;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub prev_vel: Vec2,
    pub velocity: Vec2,
    /// force
    pub force: Vec2,
    pub is_kinematic: bool,
    pub id: u32,
}

impl Point {
    // pub fn addForce(&mut self, force: Vec2){
    //     self.force += force;
    // }

    pub fn update(&mut self, dt: f64) {
        if self.is_kinematic || self.force.is_nan() {
            return;
        }
        // self.air_drag(dt);
        // self.gravity(dt);
        self.verlet_integration(dt);
    }
    pub fn new() -> Point {
        Point {
            pos: vec2(0.0, 0.0),
            prev_vel: vec2(1.0, 0.0),
            velocity: vec2(0.0, 0.0),
            // aka force
            force: vec2(0.0, 0.0),
            is_kinematic: false,
            id: 0,
        }
    }
    fn verlet_integration(&mut self, dt: f64) {
        let prev_pos = self.velocity;
        let future_acceleration = self.velocity - self.prev_vel;
        // https://en.wikipedia.org/wiki/Verlet_integration#Algorithmic_representation
        self.pos = self.pos + self.velocity * dt as f32 + self.force * (dt * dt * 0.5) as f32;
        self.velocity = (self.velocity + (self.force+future_acceleration) * (dt*0.5) as f32).clamp_length(0.0001, 100.0);
        self.prev_vel = prev_pos;
    }
    // pub fn from(pos: Vec2, velocity: Vec2,force: Vec2,is_kinematic: bool, id: u32) -> Point{
    //     Point {
    //         pos,
    //         velocity,
    //         force,
    //         is_kinematic,
    //         id,
    //     }
    // }
    pub fn air_drag(&mut self) {
        self.force = self.force.mul(0.99);
        self.velocity = self.velocity.mul(0.99);
    }
    pub fn gravity(&mut self, dt: f64) {
        self.force.y = -100.0;
    }
}
