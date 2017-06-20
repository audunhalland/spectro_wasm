use std::slice;

extern crate rustfft;

use rustfft::num_complex::Complex;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

struct Surface<'a> {
    pub buf: &'a mut [u8],
    pub width: usize,
    pub height: usize
}

impl<'a> Surface<'a> {
    pub unsafe fn new(buf: *mut u8, width: usize, height: usize) -> Surface<'a> {
        Surface {
            buf: slice::from_raw_parts_mut(buf, width * height * 4),
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
}

#[no_mangle]
pub unsafe fn add_one(ptr: *const u32) -> u32 {
    *ptr + 1
}

#[no_mangle]
pub unsafe fn draw_spectro(audio_buf: *const f32, audio_len: usize,
                           gfx_buf: *mut u8,
                           gfx_width: usize, gfx_height: usize) {
    analyze_internal(slice::from_raw_parts(audio_buf, audio_len));

    let mut surface = Surface::new(gfx_buf, gfx_width, gfx_height);
    surface.clear(Color { r: 0, g: 0, b: 0, a: 255 });
}

fn analyze_internal(buf: &[f32]) {
    println!("{} of {} samples has energy",
             buf.iter().fold(0, |count, sample| if *sample == 0.0 { count } else { count + 1 } ),
             buf.len());

    let mut input = Vec::new();
    for sample in buf {
        input.push(Complex { re: *sample, im: 0.0 });
    }
    let mut output = vec![Complex { re: 0.0, im: 0.0 }; buf.len()];

    let mut planner = rustfft::FFTplanner::new(false);
    let fft = planner.plan_fft(buf.len());
    fft.process(&mut input, &mut output);
}

// When exporting other functions explicitly, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
