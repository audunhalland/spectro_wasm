pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub struct Surface {
    pub buf: Vec<u8>,
    pub width: usize,
    pub height: usize
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Surface {
        Surface {
            buf: vec![0; width * height * 4],
            width: width,
            height: height
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

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        let i = (y * self.width + x) * 4;
        self.buf[i+0] = color.r;
        self.buf[i+1] = color.g;
        self.buf[i+2] = color.b;
        self.buf[i+3] = color.a;
    }
}
