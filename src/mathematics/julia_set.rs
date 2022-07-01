// Source: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/julia_set/src/lib.rs
// Source: https://github.com/JamesMcGuigan/ecosystem-research/blob/master/wasm/js-canvas/rust-webpack/src/julia_set.rs

use gloo_console::log;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::mathematics::complex::Complex;
use crate::services::colors::map_colorscheme;

// #[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn julia_set_canvas(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f32,
    imag: f32,
    radius: f32,
    limit: u32,
    colorscheme_fn: &Option<fn(f32) -> u32>,  // prevents #[wasm_bindgen]
) {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imag };
    let data_julia = julia_set(c, width, height, radius, limit);
    let data_color = map_colorscheme(&data_julia, colorscheme_fn);
    let data_clamped = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data_color), width, height
    ).unwrap();
    ctx.put_image_data(&data_clamped, 0.0, 0.0).ok();
}

fn julia_set(c: Complex, width: u32, height: u32, radius: f32, limit: u32) -> Vec<u32> {
    let capacity = (width * height) as usize;
    let mut data = Vec::<u32>::with_capacity(capacity);

    // Center the Julia set in the middle of the screen
    let min_side = std::cmp::min(width, height) as f32;
    let scale    = 2. * radius / min_side;
    let offset_x = width  as f32 / 2.;
    let offset_y = height as f32 / 2.;

    for y in 0..height {
        for x in 0..width {
            let z = Complex {
                real: (y as f32 - offset_y) * scale,
                imag: (x as f32 - offset_x) * scale,
            };
            let value = julia_value(z, c, limit);
            data.push(value);
        }
    }
    data
}

fn julia_value(z: Complex, c: Complex, limit: u32) -> u32 {
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
