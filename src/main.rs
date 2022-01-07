extern crate minifb;

mod paint;

use std::rc::{self, Rc};

use minifb::{Key, Window, WindowOptions};
use crate::paint::{screen, Point, color};


const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut screen = screen::new(WIDTH, HEIGHT, color::BLACK);
    screen.plot(200, 200);
    screen.line(Point { x: 0, y: 0 }, Point { x: 100, y: 100 });
    screen.line(Point { x: 100, y: 100 }, Point { x: 150, y: 250 });
    screen.move_to(Point { x: 200, y: 200 });
    screen.line_to(Point { x: 250, y: 300 });
    screen.set_color(color::Color(0, 255, 0));
    screen.rect(Point { x: 400, y: 400 }, Point { x: 450, y: 450 });

    let mut window = Window::new(
        "graph_test",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("error: {}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(screen.get_buffer(), WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("error on render: {}", e);
            });
    }
}
