use std::time::Instant;

use nannou::prelude::*;
mod spring;
mod point;
use crate::spring::Spring;
use crate::point::Point;


const REST: f32 = 1.0;
const STIFFNESS: f32 = 20.0;
const GRAVITY:f32 = -5.0;


struct Model {
    _window: window::Id,
    spring_vec: Vec<Spring>,
    point_vec: Vec<Point>,
    last_time: Instant,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    // let point1 = Point { pos: vec2(0.0,0.0), velocity: vec2(0.0,0.0), force: vec2(0.0,0.0), is_kinematic: true };
    // let point2 = Point { pos: vec2(0.0,80.0), velocity: vec2(10.0,0.0), force: vec2(0.0,0.0), is_kinematic: false};
    // let spring = Spring { point1, point2, stiffness: 0.80, distance: 0.0, rest_length: 10.0 };
    let mut spring_vec: Vec<Spring> = vec![];
    let mut point_vec: Vec<Point> = vec![];
    let last_time = Instant::now();
    for i in 1..50 {
        if i >= 2 {
            point_vec.push(Point {
                pos: vec2(0.0, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: false,
            });
            spring_vec.push(Spring {
                point1: point_vec[point_vec.len() - 2],
                point2: *point_vec.last().unwrap(),
                stiffness: STIFFNESS,
                distance: 0.0,
                rest_length: REST,
            });
            println!("{:?}", point_vec.last().unwrap());
            println!("{:?}", point_vec[point_vec.len() - 2]);
        } else {
            point_vec.push(Point {
                pos: vec2(0.0, 0.0),
                velocity: vec2(10.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: true,
            });
            point_vec.push(Point {
                pos: vec2(0.0, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: false,
            });
            spring_vec.push(Spring {
                point1: *point_vec.first().unwrap(),
                point2: *point_vec.last().unwrap(),
                stiffness: STIFFNESS,
                distance: 0.0,
                rest_length: REST,
            });
        }
    }
    Model {
        _window,
        spring_vec,
        point_vec,
        last_time,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    let dt = now.duration_since(model.last_time).secs();

    
    model.last_time = Instant::now();


    let last_pos = app.mouse.position();


    if app.mouse.buttons.left().is_down() {
        model.point_vec.last_mut().unwrap().pos = app.mouse.position();
        model.point_vec.last_mut().unwrap().velocity = Vec2::ZERO;
        model.point_vec.last_mut().unwrap().force = Vec2::ZERO;
    }
    for i in 0..model.spring_vec.len() {
        let force = model.spring_vec[i].update();
        // update the force at each point
        model.point_vec[i+1].force += force;
        model.point_vec[i].force += -force;
        // update each point
        model.point_vec[i].air_drag(dt);
        model.point_vec[i+1].air_drag(dt);
        model.point_vec[i].gravity(dt);
        model.point_vec[i+1].gravity(dt);
        model.point_vec[i+1].update(dt);
        model.point_vec[i].update(dt);
        // set them back to their place!
        model.spring_vec[i].point1 = model.point_vec[i+1];
        model.spring_vec[i].point2 = model.point_vec[i];
        
    }
    // for point in &mut model.point_vec {
    //     point.force = Vec2::ZERO;
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    
    draw.background().color(WHITE);
    for i in 0..model.spring_vec.len() {
            draw.line().end(model.spring_vec[i].point1.pos).start(model.spring_vec[i].point2.pos);
            // draw.ellipse().color(STEELBLUE).radius(10.0).xy(model.spring_vec[i].point1.pos);
            // draw.text(&i.to_string()).xy(model.spring_vec[i].point1.pos);
            // draw.ellipse().color(STEELBLUE).radius(10.0).xy(model.spring_vec[i].point2.pos);
            // draw.text(&i.to_string()).xy(model.spring_vec[i].point2.pos);

        
    }

    draw.to_frame(app, &frame).unwrap();

}
