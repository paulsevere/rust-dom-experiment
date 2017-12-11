use euclid::*;
// use euclid::TypedPoint2D as Point;

use cgmath::Vector2;

type V2 = (f32, f32);

#[derive(Copy, Clone, Serialize)]
pub struct Point {
    pos: V2,
    vel: V2,
    rad: f32,
    mass: f32,
}


js_serializable!(Point);

pub type Objs = Vec<Point>;




const G: f32 = -9.8;
static mut P_COUNT: isize = 0;
use std::ops::DerefMut;
use std::{thread, time};


fn apply_force(mut p: Point) {}




impl Point {
    pub fn new(pos: V2, vel: V2, rad: f32, mass: f32) -> Point {


        Point {
            pos: pos,
            vel: vel,
            rad: rad,
            mass: mass,
        }

    }

    pub fn speak(self) {
        js!{console.log(@{self})}
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.vel = (self.vel.0, self.vel.1 + (G * dt));
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.0 += self.vel.0 * dt;
        self.pos.1 += self.vel.1 * dt;
    }

    pub fn step(&mut self, dt: f32) {
        self.apply_gravity(dt);
        self.update(dt);
    }

    pub fn render(&self) -> String {
        format!(
            "<circle cy={y} cx={x} class=\"obj\" r={r}></circle>",
            y = self.pos.1,
            x = self.pos.0,
            r = self.rad
        )
    }
}

#[derive(Serialize)]
pub struct Points {
    ps: Vec<Point>,
}

js_serializable!(Points);


pub fn run(objs: &mut Objs) {
    let p = Point::new((500.0, 500.0), (0.0, 0.0), 5.0, 1.0);
    p.speak();
    // let mut x: isize = 10;
    // let ten_millis = time::Duration::from_millis(100);

    // while x > 0 {

    //     updateObjs(objs, 1f32);
    //     draw(objs);
    //     x -= 1;

    //     thread::sleep(ten_millis);


    // }
}

pub fn updateObjs(objs: &mut Objs, dt: f32) {
    // for mut obj in objs {
    //     obj.step(dt);
    //     // js!{
    //     //     console.log(@{obj})
    //     // };
    // }

}

pub fn draw(objs: &mut Objs) {
    //     let mut html: String = "<svg width=1000 height=1000> <g transform=\"translate(0,1000) scale(1,-1)\">"
    //         .to_string();
    //     for obj in objs {
    //         html = [html, obj.render()].concat();
    //     }
    //     html = [html, "</g></svg>".to_string()].concat();
    //     println!("{}", html);
    //     js!{
    //         let divs = document.createRange().createContextualFragment(@{html});
    //         document.body.innerHTML = "";
    //         document.body.appendChild(divs);
    //     };
}

pub fn points() -> Points {
    let data: Points = Points {
        ps: vec![
            Point::new((500.0, 500.0), (0.0, 0.0), 5.0, 1.0),
            Point::new((80.0, 789.0), (3.0, 3.0), 5.0, 1.0),
            Point::new((350.0, 900.0), (-5.0, 0.0), 5.0, 1.0),
        ],
    };
    data
}