use gfx;

use rustfft;
use rustfft::num_complex::Complex;

use std::f32;
use std::sync::Arc;

pub struct Spectro {
    bufsize: usize,
    sample_rate: f32,
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
            sample_rate: 44100.0,
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

        let energies: Vec<f32> = output.iter()
            .map(|val| val.re.powi(2) + val.im.powi(2))
            .collect();

        surface.clear(gfx::Color { r: 0, g: 0, b: 0, a: 255 });

        let graph_color = gfx::Color { r: 255, g: 255, b: 255, a: 255 };
        let mut prev_point = gfx::Point { x: 0, y: surface.height - 1 };

        let min_freq: f32 = self.sample_rate / self.bufsize as f32;
        let max_freq: f32  = self.sample_rate / 2f32;

        let mut prev_freq = min_freq;

        for x in 0..surface.width {
            let step = x as f32 / surface.width as f32;
            let freq = min_freq * (max_freq / min_freq).powf(step);

            let bin_index0 = (prev_freq / min_freq) as usize;
            let bin_index1 = (freq / min_freq) as usize;

            let bin_energy = (&energies[bin_index0 .. bin_index1 + 1])
                .iter()
                .fold(0.0, |sum, e| sum + e);
            let db = 10.0 * bin_energy.log10();

            println!("energy for x={}: {}, db: {}", x, bin_energy, db);

            let next_point = gfx::Point {
                x: x,
                y: surface.height - ((f32::max(db, 0.0) / 70.0) * surface.height as f32) as isize - 1
            };

            surface.bresenham(prev_point.clone(), next_point.clone(), graph_color.clone());

            prev_freq = freq;
            prev_point = next_point.clone();
        }
    }
}
