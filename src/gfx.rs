use bresenham;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn as_u32(&self) -> u32 {
        (self.r as u32) << 0
            | (self.g as u32) << 8
            | (self.b as u32) << 16
            | (self.a as u32) << 24
    }
}

#[derive(Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

pub struct Surface {
    pub buf: Vec<u32>,
    pub width: isize,
    pub height: isize
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Surface {
        Surface {
            buf: vec![0; width * height],
            width: width as isize,
            height: height as isize
        }
    }

    pub fn clear(&mut self, color: Color) {
        let value = color.as_u32();
        for p in &mut self.buf {
            *p = value;
        }
    }

    pub fn pixel(&mut self, point: Point, color: Color) {
        self.pixel_oob_check(point, color.as_u32())
    }

    fn pixel_oob_check(&mut self, point: Point, color: u32) {
        let i = point.y * self.width + point.x;
        if i >= 0 && i <= self.buf.len() as isize {
            self.buf[i as usize] = color;
        }
    }

    pub fn bresenham(&mut self, point0: Point, point1: Point, color: Color) {
        let value = color.as_u32();
        for (x, y) in bresenham::Bresenham::new((point0.x, point0.y), (point1.x, point1.y)) {
            self.pixel_oob_check(Point { x: x, y: y }, value);
        }
    }
}
