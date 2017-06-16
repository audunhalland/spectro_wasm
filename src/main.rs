#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_analyze']"]
extern {}

use std::slice;

extern crate rustfft;

use rustfft::num_complex::Complex;

#[no_mangle]
pub unsafe fn analyze(buf: *const f32, length: usize) {
    analyze_internal(slice::from_raw_parts(buf, length));
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

// When exporting test, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
