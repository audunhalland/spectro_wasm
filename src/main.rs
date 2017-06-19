use std::slice;

extern crate rustfft;

use rustfft::num_complex::Complex;

#[no_mangle]
pub unsafe fn add_one(ptr: *const u32) -> u32 {
    *ptr + 1
}

#[no_mangle]
pub unsafe fn draw_spectro(audio_buf: *const f32, audio_len: usize,
                           gfx_buf: *mut u8,
                           gfx_width: usize, gfx_height: usize) {
    analyze_internal(slice::from_raw_parts(audio_buf, audio_len));
    draw_black(slice::from_raw_parts_mut(gfx_buf, gfx_width * gfx_height * 4), gfx_width, gfx_height);
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

fn draw_black(buf: &mut [u8], width: usize, height: usize) {
}

// When exporting test, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
