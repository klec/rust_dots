extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;
use rand::thread_rng as random;
use std::cmp::Ordering;

struct Point {
    position: (f64, f64),
    speed: (f64, f64),
    color: [f32; 4]
}

pub struct App {
    gl: GlGraphics,
    points: Vec<Point>, 
}

impl Point {
    fn new() -> Self {
        Point { position: (Self::random(300.0, 200.0), Self::random(300.0, 200.0) as f64),
            speed: (Self::random(-10.0, 20.0), Self::random(-10.0, 20.0)),
            color: Self::randomColor() }
    }

    fn random(from:f64, size: f64) -> f64 {
        return random().gen::<f64>()*size+from;
    }

    fn randomColor() -> [f32; 4] {
        let r = random().gen::<f32>();
        let g = random().gen::<f32>();
        let b = random().gen::<f32>();
        let o = random().gen::<f32>();
        return [r,g,b,o];
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    
        let square = rectangle::square(0.0, 0.0, 2.0);
        
        let points = &self.points;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            for point in points.iter() {
                let pos : (f64, f64) = point.position;
                let transform = c.transform.trans(pos.0, pos.1);
                circle_arc(point.color, 1.0, 0.0, 1.0, square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for point in self.points.iter_mut() {
            let x = point.position.0 as u32;
            match 400.cmp(&x) {
                Ordering::Less => point.speed.0 -= 0.2,
                Ordering::Greater => point.speed.0 += 0.2,
                Ordering::Equal => {},
            }
            
            let y = point.position.1 as u32;
            match 400.cmp(&y) {
                Ordering::Less => point.speed.1 -= 0.2,
                Ordering::Greater => point.speed.1 += 0.2,
                Ordering::Equal => {},
            }

            point.position.0 += point.speed.0; 
            point.position.1 += point.speed.1;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Floating points", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut points: Vec<Point> = Vec::with_capacity(1000);
    for _ in 0..points.capacity() {
        points.push(Point::new());
    }
        
    let mut app = App {
        gl: GlGraphics::new(opengl),
        points: points,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}