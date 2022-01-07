use std::mem::swap;
use crate::paint::point::Point;
use crate::paint::color::Color;
use crate::paint::utils::*;

#[derive(Default)]
pub struct Screen {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    cursor: Point,
    cur_color: Color,
}

pub fn new(width: usize, height: usize, bg_color: Color) -> Screen {
    Screen {
        buffer: vec![bg_color.into(); width * height],
        width,
        height,
        ..Default::default()
    }
}

impl Screen {
    fn plot_helper(&mut self, x: f64, y: f64, a: f64) {
        let alpha = (a * 255.0) as u8;
        self.plot_with_color(x as usize, y as usize, self.cur_color.with_alpha(alpha))
    }

    pub fn plot(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;
        self.buffer[index] = self.cur_color.into();
    }

    pub fn plot_with_color(&mut self, x: usize, y: usize, c: Color) {
        let index = y * self.width + x;
        self.buffer[index] = c.into();
    }


    pub fn line(&mut self, p0: Point, p1: Point) {
        let mut x0 = p0.x as f64;
        let mut x1 = p1.x as f64;
        let mut y0 = p0.y as f64;
        let mut y1 = p1.y as f64;

        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        if steep {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let mut grad = 1.0;
        if dx != 0.0 {
            grad = dy / dx;
        }

        let mut xend = rnd(x0);
        let mut yend = y0 + grad * (xend - x0);
        let mut xgap = rf_part(x0 + 0.5);
        let xpx11 = xend;
        let ypx11 = yend.floor();

        if steep {
            self.plot_helper(ypx11, xpx11, rf_part(yend) * xgap);
            self.plot_helper(ypx11 + 1.0, xpx11, frac_part(yend) * xgap);
        } else {
            self.plot_helper(xpx11, ypx11, rf_part(yend) * xgap);
            self.plot_helper(xpx11, ypx11 + 1.0, frac_part(yend) * xgap);
        }

        let mut int = yend + grad;

        xend = rnd(x1);
        yend = y1 + grad * (xend - 1.0);
        xgap = frac_part(x1 + 0.5);
        let xpx12 = xend;
        let ypx12 = yend.floor();
        if steep {
            self.plot_helper(ypx12, xpx12, rf_part(yend) * xgap);
            self.plot_helper(ypx12 + 1.0, xpx12, frac_part(yend) * xgap);
        } else {
            self.plot_helper(xpx12, ypx12, rf_part(yend) * xgap);
            self.plot_helper(xpx12, ypx12 + 1.0, frac_part(yend) * xgap);
        }

        if steep {
            let mut x = xpx11 + 1.0;
            while x < (xpx12 - 1.0) {
                self.plot_helper(int.floor(), x, rf_part(int));
                self.plot_helper(int.floor() + 1.0, x, frac_part(int));
                x = x + 1.0;
                int = int + grad;
            }
        } else {
            let mut x = xpx11 + 1.0;
            while x < (xpx12 - 1.0) {
                self.plot_helper(x, int.floor(), rf_part(int));
                self.plot_helper(x, int.floor() + 1.0, frac_part(int));
                x = x + 1.0;
                int = int + grad;
            }
        }
    }

    pub fn line_to(&mut self, p: Point) {
        self.line(self.cursor, p);
        self.cursor = p;
    }

    pub fn move_to(&mut self, p: Point) {
        self.cursor = p;
    }

    pub fn set_color(&mut self, c: Color) {
        self.cur_color = c;
    }

    pub fn rect(&mut self, p0: Point, p1: Point) {
        // let (x0, x1) = if p0.x < p1.x {
        //     (p0.x, p1.x)
        // } else {
        //     (p1.x, p0.x)
        // };

        // let (y0, y1) = if p0.y < p1.y {
        //     (p0.y, p1.y)
        // } else {
        //     (p1.y, p0.y)
        // };

        let mut x = p0.x;
        let mut y = p0.y;

        while x < p1.x {
            while y < p1.y {
                self.plot(x, y);
                y = y + 1;
            }
            x = x + 1;
            y = p0.y;
        }
    }

    pub fn get_buffer(&mut self) -> &[u32] {
        &self.buffer
    }
}