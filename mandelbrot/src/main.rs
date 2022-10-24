use std::fs::File;
use std::io::prelude::*;

const HEIGHT: i32 = 256;
const WIDTH: i32 = 256;
const MAX_CCV: i32 = 255;
const MAX_ITER: i32 = 100;

/* TODO:
 * 1) Struct for Complex Nums (norm value)
 * 2) Overload + op
 * 3) Blue/Orange Mandelbrot
 * 4) Smoothing
*/ 

fn mandelbrot(c: i32) -> i32 {
    let mut point: i32 = 0;
    let mut iteration: i32 = 0;

    while point.abs() < 2 && iteration <= MAX_ITER {
        point = point * point + c;
        iteration += 1;
    }
    iteration
}

fn main() -> std::io::Result<()> {
    let mut fractal = File::create("mandelbrot.ppm")?;
    fractal.write_all(format!("P3\n {} {}\n {}\n", HEIGHT, WIDTH, MAX_CCV).as_bytes())?;
    for _i in 0..HEIGHT {
        for _j in 0..WIDTH {
            fractal.write_all(format!("30 144 255 ").as_bytes())?;
        }
    }
    Ok(())
}
