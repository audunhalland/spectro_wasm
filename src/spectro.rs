use gfx;

use rustfft;
use rustfft::num_complex::Complex;

use std::f32;
use std::sync::Arc;

pub struct Spectro {
    bufsize: usize,
    window: Vec<f32>,
    fmin: f32,
    fmm_ratio: f32,
    fft: Arc<rustfft::FFT<f32>>
}

impl Spectro {
    pub fn new(bufsize: usize) -> Spectro {
        let window = (0..bufsize)
            .map(|i| ((f32::consts::PI * i as f32) / (bufsize - 1) as f32).sin().powi(2))
            .collect();
        let mut planner = rustfft::FFTplanner::new(false);

        const SAMPLE_RATE: f32 = 44100.0;
        let fmin = SAMPLE_RATE / bufsize as f32;
        let fmax = SAMPLE_RATE / 2f32;

        Spectro {
            bufsize: bufsize,
            window: window,
            fmin: fmin,
            fmm_ratio: fmax / fmin,
            fft: planner.plan_fft(bufsize)
        }
    }

    pub fn draw(&self, signal: &[f32], mut surface: &mut gfx::Surface) {
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

        for mag in [10.0, 100.0, 1000.0, 10000.0].iter() {
            self.draw_freq_marker(*mag, gfx::Color { r: 100, g: 255, b: 255, a: 255 }, &mut surface);
            for i in 2..10 {
                let f = mag * i as isize as f32;
                self.draw_freq_marker(f, gfx::Color { r: 100, g: 100, b: 100, a: 255 }, &mut surface);
            }
        }

        self.draw_energy(&energies, &mut surface);
    }

    fn draw_energy(&self, energies: &[f32], surface: &mut gfx::Surface) {
        let graph_color = gfx::Color { r: 255, g: 255, b: 255, a: 255 };
        let mut prev_point = gfx::Point { x: 0, y: surface.height - 1 };

        let mut prev_freq = self.fmin;

        for x in 0..surface.width {
            let step = x as f32 / surface.width as f32;
            let freq = self.fmin * self.fmm_ratio.powf(step);

            let bin_index0 = (prev_freq / self.fmin) as usize;
            let bin_index1 = (freq / self.fmin) as usize;

            let bin_energy = (&energies[bin_index0 .. bin_index1 + 1])
                .iter()
                .fold(0.0, |sum, e| sum + e);
            let db = 10.0 * bin_energy.log10();

            let next_point = gfx::Point {
                x: x,
                y: surface.height - ((f32::max(db, 0.0) / 70.0) * surface.height as f32) as isize - 1
            };

            surface.bresenham(prev_point.clone(), next_point.clone(), graph_color.clone());

            prev_freq = freq;
            prev_point = next_point.clone();
        }
    }

    fn draw_freq_marker(&self, freq: f32, color: gfx::Color, surface: &mut gfx::Surface) {
        let scaled = (freq / self.fmin).ln() / self.fmm_ratio.ln();
        let y0 = 0;
        let y1 = surface.height - 1;
        if scaled >= 0.0 && scaled < 1.0 {
            let x = (scaled * surface.width as f32) as isize;

            surface.bresenham(gfx::Point { x: x, y: y0 },
                              gfx::Point { x: x, y: y1 },
                              color);
        }
    }
}
