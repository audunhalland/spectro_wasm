mod gfx;

use std::f32;
use std::slice;

extern crate rustfft;

use rustfft::num_complex::Complex;

#[no_mangle]
pub unsafe fn add_one(ptr: *const u32) -> u32 {
    *ptr + 1
}

#[no_mangle]
pub unsafe fn create_surface(width: usize, height: usize) -> *mut gfx::Surface {
    Box::into_raw(Box::new(gfx::Surface::new(width, height)))
}

#[no_mangle]
pub unsafe fn surface_buf(surface: *mut gfx::Surface) -> *const u8 {
    (*surface).buf.as_ptr()
}

#[no_mangle]
pub unsafe fn draw_spectro(audio_buf: *const f32, audio_len: usize,
                           surface: *mut gfx::Surface) {
    draw_spectro_internal(slice::from_raw_parts(audio_buf, audio_len),
                          &mut *surface);
}

fn draw_spectro_internal(buf: &[f32], surface: &mut gfx::Surface) {
    println!("surface: {}, {}, {}", surface.buf.len(), surface.width, surface.height);

    /*
    println!("{} of {} samples has energy",
             buf.iter().fold(0, |count, sample| if *sample == 0.0 { count } else { count + 1 } ),
             buf.len());
    */

    let mut input = Vec::new();
    for sample in buf {
        input.push(Complex { re: *sample, im: 0.0 });
    }
    let mut output = vec![Complex { re: 0.0, im: 0.0 }; buf.len()];

    let mut planner = rustfft::FFTplanner::new(false);
    let fft = planner.plan_fft(buf.len());
    fft.process(&mut input, &mut output);

    let mut dbs = Vec::new();

    for value in &output {
        dbs.push(10f32 * (value.re * value.re + value.im * value.im).log10() / 10f32.log10());
    }

    let min_db = dbs.iter().cloned().fold(f32::NAN, f32::min);
    let max_db = dbs.iter().cloned().fold(f32::NAN, f32::max);

    let min_db_limit = 0f32;
    let max_db_limit = 100f32;

    surface.clear(gfx::Color { r: 0, g: 0, b: 0, a: 255 });

    println!("min, max db = {}, {}", min_db, max_db);

    for x in 0..surface.width {
        let mut db = dbs[((x as f64 / surface.width as f64) * output.len() as f64) as usize];
        if db < min_db_limit {
            db = min_db_limit;
        } else if db > max_db_limit {
            db = max_db_limit;
        }

        let scaled = (db - min_db_limit) / (max_db_limit - min_db_limit);
        let y = surface.height - (scaled * surface.height as f32) as usize - 1;

        if x < 3 {
            println!("x={} index={} db={} scaled={} y={}",
                     x,
                     ((x as f64 / surface.width as f64) * output.len() as f64) as usize,
                     db,
                     scaled,
                     y);
        }


        surface.point(x, y, gfx::Color { r: 255, g: 255, b: 255, a: 255 });
    }
}

// When exporting other functions explicitly, main is not exported at all.
fn main() {
    println!("rust: main. NO CALLS HERE");
}
