use std::borrow::BorrowMut;
use std::time::Instant;

use nannou::prelude::*;
mod point;
mod spring;
use crate::point::Point;
use crate::spring::Spring;

const REST: f32 = 10.0;
const STIFFNESS: f32 = 100.0;
const HOW_MANY: i32 = 20;

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
    // add first points
    let mut id = 0;
    // point_vec.push(Point {
    // pos: vec2(0.0, 0.0),
    // velocity: vec2(10.0, 0.0),
    // force: vec2(0.0, 0.0),
    // is_kinematic: true,
    // id,
    // });
    // id+=1;
    // point_vec.push(Point {
    //     pos: vec2(0.0, -REST),
    //     velocity: vec2(0.0, 0.0),
    //     force: vec2(0.0, 0.0),
    //     is_kinematic: true,
    //     id,
    // });
    // id+=1;
    // spring_vec.push(Spring {
    //     point1: *point_vec.first().unwrap(),
    //     point2: *point_vec.last().unwrap(),
    //     stiffness: STIFFNESS,
    //     distance: 0.0,
    //     rest_length: REST,
    // });
    // add all other points
    for i in 0..HOW_MANY {
        id += 1;
        if i == 0 {

            point_vec.push(Point {
                pos: vec2(0.0, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: true,
                id,
            });
        } else {
            point_vec.push(Point {
                pos: vec2(0.0, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: false,
                id,
            });

        }
        for j in 1..HOW_MANY + 1 {
            id += 1;
            if i == 0 {
                point_vec.push(Point {
                    pos: vec2((j as f32 * -REST) as f32, (i as f32 * -REST) as f32),
                    velocity: vec2(0.0, 0.0),
                    force: vec2(0.0, 0.0),
                    is_kinematic: true,
                    id,
                });
                continue;
            }
            point_vec.push(Point {
                pos: vec2((j as f32 * -REST) as f32, (i as f32 * -REST) as f32),
                velocity: vec2(0.0, 0.0),
                force: vec2(0.0, 0.0),
                is_kinematic: false,
                id,
            });
        }
    }
    // TODO: connect all springs to every point!
    // above = -rest
    // below = rest
    // right = -1
    // left = 1
    // we only need left and below.
    for i in 0..point_vec.len() {
        // up
        if (i as i32 - HOW_MANY) > 0 {
            spring_vec.push(Spring {
                point1: point_vec[i],
                point2: point_vec[i - HOW_MANY as usize - 1],
                stiffness: STIFFNESS,
                distance: 0.0,
                rest_length: REST,
            });
            if point_vec[i].pos.x != point_vec[i - HOW_MANY as usize - 1].pos.x {
                println!("go up")
            }
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
            // if point_vec[i].pos.y != point_vec[i + 1].pos.y {
            //     println!("go left, point 1: {}, point 2 {}", point_vec[i].pos.y, point_vec[i + 1].pos.y)
            // }
        }
        // if(point_vec[i].pos == vec2(0.0, -10.0)){
        //     println!("found point == -10")
        // }
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

    if app.mouse.buttons.left().is_down() {
        let mut closest: f32 = f32::MAX;
        let mut closest_point_i:usize = 0;
        let mut drag_point:&mut Point;
        for point in 0..model.point_vec.len() {
            let dist = app.mouse.position().distance(model.point_vec[point].pos);

            if dist < closest {
                closest = dist;
                closest_point_i = point.clone();
            }
        }

        drag_point = &mut model.point_vec[closest_point_i];
        drag_point.pos = app.mouse.position();
        
        // closest_point.pos = app.mouse.position();
        // model.point_vec.last_mut().unwrap().pos = app.mouse.position();
        // model.point_vec.last_mut().unwrap().velocity = Vec2::ZERO;
        // model.point_vec.last_mut().unwrap().force = Vec2::ZERO;
    }
    for i in 0..model.spring_vec.len() {
        let force = model.spring_vec[i].update();
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

        model.point_vec[index1].force += force;
        model.point_vec[index2].force += -force;
        // update each point
        model.point_vec[index1].air_drag();
        model.point_vec[index2].air_drag();
        model.point_vec[index1].gravity(dt);
        model.point_vec[index2].gravity(dt);
        model.point_vec[index1].update(dt);
        model.point_vec[index2].update(dt);
        // set them back to their place!
        model.spring_vec[i].point1 = model.point_vec[index1];
        model.spring_vec[i].point2 = model.point_vec[index2];
    }
    for point in &mut model.point_vec {
        point.force = Vec2::ZERO;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    for i in 0..model.spring_vec.len() {
        draw.line()
            .end(model.spring_vec[i].point1.pos)
            .start(model.spring_vec[i].point2.pos);
        // draw.ellipse().color(STEELBLUE).radius(10.0).xy(model.spring_vec[i].point1.pos);
        // draw.text(&i.to_string()).xy(model.spring_vec[i].point1.pos);
        // draw.ellipse().color(STEELBLUE).radius(10.0).xy(model.spring_vec[i].point2.pos);
        // draw.text(&i.to_string()).xy(model.spring_vec[i].point2.pos);
    }
    // for i in model.point_vec.clone().into_iter(){
    //     draw.ellipse().color(STEELBLUE).radius(10.0).xy(i.pos);

    // }

    draw.to_frame(app, &frame).unwrap();
}

// fn find_closest_point(mouse_pos: Vec2){

// }
