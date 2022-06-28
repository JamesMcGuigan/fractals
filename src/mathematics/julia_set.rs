// Source: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/julia_set/src/lib.rs
// Source: https://github.com/JamesMcGuigan/ecosystem-research/blob/master/wasm/js-canvas/rust-webpack/src/julia_set.rs

// use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use crate::mathematics::complex::Complex;

#[wasm_bindgen]
pub fn draw_julia_set(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    let data_julia = get_julia_set(width, height, c);
    let data_clamped = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data_julia), width, height
    ).unwrap();
    ctx.put_image_data(&data_clamped, 0.0, 0.0).ok();
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for y in 0..height {
        for x in 0..width {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

