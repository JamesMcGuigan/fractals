// Source: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/julia_set/src/lib.rs
// Source: https://github.com/JamesMcGuigan/ecosystem-research/blob/master/wasm/js-canvas/rust-webpack/src/julia_set.rs

use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::mathematics::complex::Complex;

#[wasm_bindgen]
pub fn draw_julia_set(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imag: f64,
    radius: f64,
) {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imag };
    let data_julia = get_julia_set(c, width, height, radius);
    let data_clamped = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data_julia), width, height
    ).unwrap();
    ctx.put_image_data(&data_clamped, 0.0, 0.0).ok();
}

fn get_julia_set(c: Complex, width: u32, height: u32, radius: f64) -> Vec<u8> {
    let iter_limit = 32;

    let capacity = (width * height * 4) as usize;
    let mut data = Vec::with_capacity(capacity);

    // Center the Julia set in the middle of the screen
    let min_side = std::cmp::min(width, height);
    let scale    = (2. * radius as f64) / (min_side as f64);
    let offset_x = width  as f64 / 2.;
    let offset_y = height as f64 / 2.;

    for y in 0..height {
        for x in 0..width {
            let z = Complex {
                real: (y as f64 - offset_y) * scale,
                imag: (x as f64 - offset_x) * scale,
            };
            let iter_index = get_iter_index(z, c, iter_limit);
            let iter_value = 255. * (iter_index as f64 / iter_limit as f64).powf(0.25);
            data.push((iter_value %  64. * 4.) as u8);  // RED
            data.push((iter_value % 128. * 2.) as u8);  // BLUE
            data.push((iter_value % 255. * 1.) as u8);  // GREEN
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex, limit: u32) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < limit {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

