use gfx;

use rustfft;
use rustfft::num_complex::Complex;

use std::f32;
use std::sync::Arc;

pub struct Spectro {
    bufsize: usize,
    fft: Arc<rustfft::FFT<f32>>
}

impl Spectro {
    pub fn new(bufsize: usize) -> Spectro {
        let mut planner = rustfft::FFTplanner::new(false);
        let mut window = vec![0f32; bufsize];

        Spectro {
            bufsize: bufsize,
            fft: planner.plan_fft(bufsize)
        }
    }

    pub fn draw(&self, signal: &[f32], surface: &mut gfx::Surface) {
        if signal.len() != self.bufsize {
            panic!()
        }

        let mut input = Vec::with_capacity(self.bufsize);
        for sample in signal {
            input.push(Complex { re: *sample, im: 0.0 });
        }
        let mut output = vec![Complex { re: 0.0, im: 0.0 }; self.bufsize];

        self.fft.process(&mut input, &mut output);

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
}
