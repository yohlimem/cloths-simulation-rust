use std::borrow::BorrowMut;
use std::ops::Add;
use std::time::Instant;

use nannou::prelude::*;
mod point;
mod spring;
use crate::point::Point;
use crate::spring::Spring;

const REST: f32 = 30.0;
const STIFFNESS: f32 = 1.0;
const HOW_MANY: i32 = 10;
const OFFSETS:[i32;2] = [-1, -HOW_MANY];

struct Model {
    _window: window::Id,
    spring_vec: Vec<Spring>,
    point_vec: Vec<Point>,
    last_time: Instant,
    last_point: usize,
    last_mouse_pos: Vec2,
    once: bool,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    // let point1 = Point { pos: vec2(0.0,0.0), velocity: vec2(0.0,0.0), force: vec2(0.0,0.0), is_kinematic: true };
    // let point2 = Point { pos: vec2(0.0,80.0), velocity: vec2(10.0,0.0), force: vec2(0.0,0.0), is_kinematic: false};
    // let spring = Spring { point1, point2, stiffness: 0.80, distance: 0.0, rest_length: 10.0 };
    // let mut spring_vec: Vec<Spring> = vec![];
    // let mut point_vec: Vec<Point> = vec![];
    let last_time = Instant::now();
    // add first points
    let arrays = create_grid();
    // let arrays = create_string();
    
    let last_point = 0;
    let last_mouse_pos = vec2(0.0, 0.0);
    let once = false;
    Model {
        _window,
        spring_vec: arrays.0,
        point_vec: arrays.1,
        last_time,
        last_point,
        last_mouse_pos,
        once,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    let dt = now.duration_since(model.last_time).secs();
    model.last_time = Instant::now();

    
    if app.mouse.buttons.left().is_down() && !model.once {
        let mut closest_point_i: usize = 0;
        model.last_mouse_pos = app.mouse.position();
        model.once = true;
        let mut closest: f32 = f32::MAX;
        for point in 0..model.point_vec.len() {
            let dist = model.last_mouse_pos.distance(model.point_vec[point].pos);

            if dist < closest {
                closest = dist;
                closest_point_i = point.clone();
            }
        }
        model.last_point = closest_point_i;
    } else if model.once && app.mouse.buttons.left().is_up() {
        model.once = false;
    }
    if app.mouse.buttons.left().is_down() {
        model.point_vec[model.last_point].pos = app.mouse.position();
    }
    for _ in 0..50{
        for i in 0..model.point_vec.len(){
            for j in OFFSETS{
                if i as i32 + j >= model.point_vec.len() as i32 || i as i32 + j < 0 || (j == -1 && (model.point_vec[i].id - 1) as i32 % HOW_MANY == 0) {
                    continue;
                }
                let index = i as i32 + j;
                // let point1_pos = model.point_vec[i].pos;
                let point2 = model.point_vec[i];
                let point1 = model.point_vec[index as usize];
                let new_pos = Spring::update(REST ,point1, point2);
                if!model.point_vec[index as usize].is_kinematic {
                    model.point_vec[index as usize].pos += new_pos;

                }
                if !model.point_vec[i].is_kinematic{
                    model.point_vec[i].pos += -new_pos;
                }
                // if model.point_vec[i].is_kinematic {
                //     model.point_vec[i].pos = point1_pos;
                // }

                // model.point_vec[index as usize].update(dt as f64);
                // model.point_vec[i].update(dt as f64);
                // model.point_vec[index as usize].velocity = new_pos / dt as f32;
                // model.point_vec[i].velocity = -new_pos / dt as f32;
                // model.point_vec[index as usize].update(dt as f64);
                // model.point_vec[i].update(dt as f64);
                
            }

        }

    }
    for i in 0..model.spring_vec.len() {
        // update the force at each point
        // println!("i: {i}, spring: {:?}, pos 2: {}", model.spring_vec[i], model.spring_vec[i].point2.pos);
        let index1 = model
            .point_vec
            .iter()
            .position(|&r| r.id == model.spring_vec[i].point1.id)
            .expect("couldnt find the point1");
        let index2 = model
            .point_vec
            .iter()
            .position(|&r| r.id == model.spring_vec[i].point2.id)
            .expect("couldnt find the point2");


    //===================================================================================\\
    model.point_vec[index1].gravity(dt);
    model.point_vec[index2].gravity(dt);
    model.point_vec[index1].update(dt as f64);
    model.point_vec[index2].update(dt as f64);
    model.spring_vec[i].point1 = model.point_vec[index1];
    model.spring_vec[i].point2 = model.point_vec[index2];
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();


    draw.background().color(WHITE);
    for i in 0..model.spring_vec.len() {
        draw.line()
            .end(model.spring_vec[i].point1.pos)
            .start(model.spring_vec[i].point2.pos);
        // draw.ellipse().color(BLACK).radius(4.0).xy(model.spring_vec[i].point1.pos);
        // draw.ellipse().color(BLACK).radius(4.0).xy(model.spring_vec[i].point2.pos);
    }
    // draw.polygon().points(model.point_vec.clone().into_iter().map(|p| p.pos).collect::<Vec<Vec2>>()).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

fn create_grid() -> (Vec<Spring>, Vec<Point>){
    let mut point_vec = vec![];
    let mut spring_vec = vec![];
    let mut id = 0;
    for i in 0..HOW_MANY {
        id += 1;
        if i == 0 {

            point_vec.push(Point {
                pos: vec2(0.0, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                prev_vel: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: true,
                id,
            });
        } else {
            point_vec.push(Point {
                pos: vec2(0.0, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                prev_vel: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: false,
                id,
            });

        }
        for j in 1..HOW_MANY {
            id += 1;
            if i == 0 {
                point_vec.push(Point {
                    pos: vec2((j as f32 * -REST) as f32, (i as f32 * -REST) as f32),
                    velocity: vec2(0.0, 0.0),
                    prev_vel: vec2(0.0, 0.0),
                    force: vec2(0.0, 0.0),
                    is_kinematic: true,
                    id,
                });
                continue;
            }
            point_vec.push(Point {
                pos: vec2((j as f32 * -REST) as f32, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                prev_vel: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: false,
                id,
            });
        }
    }
    for i in 0..point_vec.len() {
        // up
        if (i as i32 - HOW_MANY) >= 0 {
            spring_vec.push(Spring {
                point1: point_vec[i],
                point2: point_vec[i - HOW_MANY as usize],
                stiffness: STIFFNESS,
                distance: 0.0,
                rest_length: REST,
            });
        }
        if (i as i32 + 1) < point_vec.len() as i32 && point_vec[i].pos.y == point_vec[i + 1].pos.y {
            // left
            spring_vec.push(Spring {
                point1: point_vec[i],
                point2: point_vec[i + 1],
                stiffness: STIFFNESS,
                distance: 0.0,
                rest_length: REST,
            });
        }
    }
    return (spring_vec, point_vec);
}

fn create_string() -> (Vec<Spring>, Vec<Point>){
    let mut point_vec = vec![];
    let mut spring_vec = vec![];
    let mut id = 0;
    for i in 0..HOW_MANY {
        point_vec.push(Point {
            pos: vec2(0.0, (i as f32 * -REST) as f32),
            prev_vel: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            force: vec2(0.0, 0.0),
            is_kinematic: false,
            id,
        });
        id+=1;
    }
    for i in 0..HOW_MANY - 1 {
        spring_vec.push(Spring {
            point1: point_vec[i as usize],
            point2: point_vec[i as usize + 1],
            stiffness: STIFFNESS,
            distance: 0.0,
            rest_length: REST,
        });
    }
    return (spring_vec, point_vec);
}


// fn find_closest_point(mouse_pos: Vec2){

// }
