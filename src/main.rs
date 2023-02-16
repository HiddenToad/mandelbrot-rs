use image::ColorType::Rgb8;
use rayon::prelude::*;

const IMG_W: u32 = 2500;
const IMG_H: u32 = 2500;
const MAX_DEPTH: u32 = 1100;
const SAVE_PATH: &'static str = "/Users/judah/mandelbrot.png";

fn compute_mandelbrot(x0: u32, y0: u32) -> u32 {
    let mut x = 0.;
    let mut y = 0.;
    let cx = 3.0 * (x0 as f64 / IMG_W as f64) - 2.0;
    let cy = 3.0 * (y0 as f64 / IMG_H as f64) - 1.5;
    let mut i = 0;
    while x * x + y * y <= 4. && i < MAX_DEPTH {
        let xtemp = x * x - y * y + cx;
        y = 2. * x * y + cy;
        x = xtemp;
        i += 1;
    }
    i
}

fn main() {
    let mut buf = Box::new([0; (IMG_W * IMG_H * 3) as usize]);

    buf.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
        let x = i as u32 % IMG_W;
        let y = i as u32 / IMG_W;
        let rgb = if compute_mandelbrot(x, y) >= MAX_DEPTH {
            [0, 0, 0]
        } else {
            [255, 255, 255]
        };
        pixel.copy_from_slice(&rgb);
    });

    image::save_buffer(SAVE_PATH, &*buf, IMG_W, IMG_H, Rgb8).expect("Error saving image");
}
