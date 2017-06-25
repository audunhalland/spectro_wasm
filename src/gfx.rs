use bresenham;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

pub struct Surface {
    pub buf: Vec<u8>,
    pub width: isize,
    pub height: isize
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Surface {
        Surface {
            buf: vec![0; width * height * 4],
            width: width as isize,
            height: height as isize
        }
    }

    pub fn clear(&mut self, color: Color) {
        let mut i = 0;
        for _ in 0..(self.width * self.height) {
            self.buf[i+0] = color.r;
            self.buf[i+1] = color.g;
            self.buf[i+2] = color.b;
            self.buf[i+3] = color.a;
            i += 4;
        }
    }

    pub fn pixel(&mut self, point: Point, color: Color) {
        let i = (point.y * self.width + point.x) * 4;
        if i >= 0 && i <= self.buf.len() as isize {
            let iu = i as usize;
            self.buf[iu+0] = color.r;
            self.buf[iu+1] = color.g;
            self.buf[iu+2] = color.b;
            self.buf[iu+3] = color.a;
        }
    }

    pub fn bresenham(&mut self, point0: Point, point1: Point, color: Color) {
        for (x, y) in bresenham::Bresenham::new((point0.x, point0.y), (point1.x, point1.y)) {
            self.pixel(Point { x: x, y: y }, color.clone());
        }
    }
}
