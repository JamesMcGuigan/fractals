// Source: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/julia_set/src/lib.rs
// Source: https://github.com/JamesMcGuigan/ecosystem-research/blob/master/wasm/js-canvas/rust-webpack/src/julia_set.rs

use gloo_console::log;
use num_complex::Complex;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use crate::services::colorschemes::ColorScheme;

use crate::services::vectors::{map_colorscheme, vec_u32_to_u8};

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
    colorscheme: ColorScheme,  // prevents #[wasm_bindgen]
) {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex::new(real, imag);
    let data_julia: Vec<u32> = julia_set(c, width, height, radius, limit);
    let data_color: Vec<u32> = map_colorscheme(&data_julia, colorscheme);
    let data_color_u8: Vec<u8> = vec_u32_to_u8(&data_color);
    let data_clamped = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data_color_u8), width, height
    ).unwrap();
    {
        let _data_julia_min: u32  = *data_julia.iter().min().unwrap();
        let _data_julia_max: u32  = *data_julia.iter().max().unwrap();
        let _data_julia_mean: u32 =  data_julia.iter().sum::<u32>() / data_julia.len() as u32;
        let _data_color_min: u8   = *data_color_u8.iter().min().unwrap();
        let _data_color_max: u8   = *data_color_u8.iter().max().unwrap();
        let _data_color_mean: u32 =  data_color_u8.iter().map(|&e| e as u32).sum::<u32>() / data_color.len() as u32;
        log!(format!("data_julia({_data_julia_min}, {_data_julia_mean}, {_data_julia_mean})"));
        log!(format!("data_color({_data_color_min}, {_data_color_mean}, {_data_color_mean})"));
    }
    ctx.put_image_data(&data_clamped, 0.0, 0.0).ok();
}

pub fn julia_set(c: Complex<f32>, width: u32, height: u32, radius: f32, limit: u32) -> Vec<u32> {
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
                re: (y as f32 - offset_y) * scale,
                im: (x as f32 - offset_x) * scale,
            };
            let value = julia_value(z, c, limit);
            data.push(value);
        }
    }
    data
}

pub fn julia_value(z: Complex<f32>, c: Complex<f32>, limit: u32) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < limit {
        if z.norm() > 2.0 {
            break;
        }
        z = (z * z) + c;
        iter_index += 1;
    }
    iter_index
}
