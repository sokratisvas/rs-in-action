use std::fs::File;
use std::io::prelude::*;

const HEIGHT: i32 = 612;
const WIDTH: i32 = 612;
const MAX_CCV: i32 = 255;
const MAX_ITER: i32 = 100;
const UPPER_REAL: i32 = 1;
const LOW_REAL: i32 = -2;
const UPPER_IMAG: i32 = 1;
const LOW_IMAG: i32 = -1;

#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex {
            real: real,
            imag: imag
        }
    }

    fn norm(self) -> f64 {
        ((self.real).powi(2) + (self.real).powi(2)).sqrt()
    }

    fn set_coord(&mut self, x: f64, y: f64) {
        self.real = x;
        self.imag = y;
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, complex: Complex) -> Complex {
        Complex {
            real: self.real + complex.real,
            imag: self.imag + complex.imag
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, complex: Complex) -> Complex {
        Complex {
            real: self.real * complex.real - self.imag * complex.imag,
            imag: self.real * complex.imag + self.imag * complex.real
        }
    }
}

fn mandelbrot(c: Complex) -> i32 {
    let mut point: Complex = Complex::new(0.0, 0.0);
    let mut iteration: i32 = 0;

    while point.norm() < 2.0 && iteration < MAX_ITER {
        point = point * point + c;
        iteration += 1;
    }

    iteration
}

fn main() -> std::io::Result<()> {
    let mut fractal = File::create("mandelbrot.ppm")?;
    fractal.write_all(format!("P3\n {} {}\n {}\n", HEIGHT, WIDTH, MAX_CCV).as_bytes())?;
    let mut point: Complex = Complex::new(0.0, 0.0);
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            point.set_coord(
                LOW_REAL as f64 + (i as f64 / WIDTH as f64) * (UPPER_REAL - LOW_REAL) as f64,
                LOW_IMAG as f64 + (j as f64 / HEIGHT as f64) * (UPPER_IMAG - LOW_IMAG) as f64
            );
            let iters = mandelbrot(point);
            let color = 255 - iters * ((255 as f64) / (MAX_ITER as f64)) as i32;
            fractal.write_all(format!("{0} {0} {0} ", color).as_bytes())?;
        }
    }
    Ok(())
}
