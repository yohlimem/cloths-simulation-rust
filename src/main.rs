use std::time::Instant;

use nannou::prelude::*;
mod spring;
mod point;
use crate::spring::Spring;
use crate::point::Point;


struct Model {
    _window: window::Id,
    spring: Spring,
    point1: Point,
    point2: Point,
    last_time: Instant,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let point1 = Point { pos: vec2(0.0,0.0), velocity: vec2(0.0,0.0), force: vec2(0.0,0.0), is_kinematic: true };
    let point2 = Point { pos: vec2(0.0,80.0), velocity: vec2(10.0,0.0), force: vec2(0.0,0.0), is_kinematic: false};
    let spring = Spring { point1, point2, stiffness: 0.80, distance: 0.0, rest_length: 10.0 };
    let last_time = Instant::now();
    Model {
        _window,
        spring,
        point1,
        point2,
        last_time,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    let dt = now.duration_since(model.last_time).secs() * 10.0;
    model.last_time = Instant::now();
    model.spring.update();
    model.spring.point1.update(dt);
    model.spring.point2.update(dt);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    
    draw.background().color(WHITE);
    
    draw.ellipse().color(STEELBLUE).radius(10.0).xy(model.spring.point1.pos);
    draw.ellipse().color(STEELBLUE).radius(10.0).xy(model.spring.point2.pos);
    draw.line().end(model.spring.point1.pos).start(model.spring.point2.pos);

    draw.to_frame(app, &frame).unwrap();

}
