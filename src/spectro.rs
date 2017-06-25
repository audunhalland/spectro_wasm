use gfx;

use rustfft;
use rustfft::num_complex::Complex;

use std::f32;
use std::sync::Arc;

pub struct Spectro {
    bufsize: usize,
    window: Vec<f32>,
    fft: Arc<rustfft::FFT<f32>>
}

impl Spectro {
    pub fn new(bufsize: usize) -> Spectro {
        let window = (0..bufsize)
            .map(|i| ((f32::consts::PI * i as f32) / (bufsize - 1) as f32).sin().powi(2))
            .collect();
        let mut planner = rustfft::FFTplanner::new(false);

        Spectro {
            bufsize: bufsize,
            window: window,
            fft: planner.plan_fft(bufsize)
        }
    }

    pub fn draw(&self, signal: &[f32], surface: &mut gfx::Surface) {
        if signal.len() != self.bufsize {
            panic!()
        }

        // Multiply signal with the window function
        let mut input: Vec<Complex<f32>> = signal.iter().zip(&self.window)
            .map(|(sample, window_n)| Complex { re: sample * window_n, im: 0.0 })
            .collect();
        let mut output = vec![Complex { re: 0.0, im: 0.0 }; self.bufsize];

        self.fft.process(&mut input, &mut output);

        let dbs: Vec<f32> = output.iter()
            .map(|val| 10f32 * (val.re.powi(2) + val.im.powi(2)).log10() / 10f32.log10())
            .collect();

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
