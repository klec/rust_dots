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
    points: [Point; 2], 
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    
        let square = rectangle::square(0.0, 0.0, 5.0);
        
        let pos : (f64, f64) = self.points[0].position;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let transform = c.transform.trans(pos.0, pos.1);
            circle_arc(RED, 3.0, 0.0, 10.0, square, transform, gl)
        });
    }

    fn update(&mut self, args: &UpdateArgs) { //@todo process array of points
        
        let x = self.points[0].position.0 as u32;
        let y = self.points[0].position.1 as u32;
        
        match 200.cmp(&x) {
            Ordering::Less => self.points[0].speed.0 -= 0.2,
            Ordering::Greater => self.points[0].speed.0 += 0.2,
            Ordering::Equal => {},
        }

        match 200.cmp(&y) {
            Ordering::Less => self.points[0].speed.1 -= 0.2,
            Ordering::Greater => self.points[0].speed.1 += 0.2,
            Ordering::Equal => {},
        }

        self.points[0].position.0 += self.points[0].speed.0; 
        self.points[0].position.1 += self.points[0].speed.1;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Floating points", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
        let point1:Point = Point {
            position: (random().gen_range(1..100) as f64, random().gen_range(1..100) as f64),
            speed: (random().gen_range(-1..1) as f64, random().gen_range(-1..1) as f64),
            color: [1.0, 0.0, 0.0, 1.0]
        };
        let point2:Point = Point {
            position: (random().gen_range(1..100) as f64, random().gen_range(1..100) as f64),
            speed: (random().gen_range(-1..1) as f64, random().gen_range(-1..1) as f64),
            color: [1.0, 0.0, 0.0, 1.0]
        };
        
    let mut app = App {
        gl: GlGraphics::new(opengl),
        points: [point1, point2],
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