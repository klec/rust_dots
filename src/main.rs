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

struct Vector {
    x: i64,
    y: i64
}

pub struct App {
    gl: GlGraphics,
    position: Vector,
    speed: (f64, f64)
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    
        let square = rectangle::square(0.0, 0.0, 5.0);
        
        let x: f64 = self.position.x as f64;
        let y: f64 = self.position.y as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let transform = c.transform.trans(x, y);
            // rectangle(RED, square, transform, gl);
            circle_arc(RED, 3.0, 0.0, 10.0, square, transform, gl)
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        match 200.cmp(&self.position.x) {
            Ordering::Less => self.speed.0 -= 0.2,
            Ordering::Greater => self.speed.0 += 0.2,
            Ordering::Equal => {},
        }
        match 200.cmp(&self.position.y) {
            Ordering::Less => self.speed.1 -= 0.2,
            Ordering::Greater => self.speed.1 += 0.2,
            Ordering::Equal => {},
        }
        self.position.x += self.speed.0 as i64;
        self.position.y += self.speed.1 as i64;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        position: Vector {x: random().gen_range(1..100), y: random().gen_range(1..100)},
        speed: (random().gen_range(-1..1) as f64, random().gen_range(-1..1) as f64)
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