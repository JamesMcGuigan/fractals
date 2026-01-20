use gloo_console::log;
use num_complex::Complex;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use crate::services::colorschemes::ColorScheme;
use crate::services::vectors::{map_colorscheme, vec_u32_to_u8};

#[allow(clippy::too_many_arguments)]
pub fn mandelbrot_set_canvas(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    center_re: f64,
    center_im: f64,
    radius: f64,
    limit: u32,
    colorscheme: ColorScheme,
) {
    let data_mandel: Vec<f64> = mandelbrot_set(width, height, center_re, center_im, radius, limit);
    let data_color: Vec<u32> = map_colorscheme(&data_mandel, colorscheme);
    let data_color_u8: Vec<u8> = vec_u32_to_u8(&data_color);
    let data_clamped = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data_color_u8), width, height
    ).unwrap();
    
    {
        let _data_mandel_min: f64  = *data_mandel.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let _data_mandel_max: f64  = *data_mandel.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let _data_mandel_mean: f64 =  data_mandel.iter().sum::<f64>() / data_mandel.len() as f64;
        log!(format!("data_mandel({_data_mandel_min:.1}, {_data_mandel_mean:.1}, {_data_mandel_max:.1})"));
    }
    
    ctx.put_image_data(&data_clamped, 0.0, 0.0).ok();
}

pub fn mandelbrot_set(width: u32, height: u32, center_re: f64, center_im: f64, radius: f64, limit: u32) -> Vec<f64> {
    let capacity = (width * height) as usize;
    let mut data = Vec::<f64>::with_capacity(capacity);

    let min_side = std::cmp::min(width, height) as f64;
    let scale    = 2. * radius / min_side;
    let offset_x = width  as f64 / 2.;
    let offset_y = height as f64 / 2.;

    for y in 0..height {
        for x in 0..width {
            let c = Complex {
                re: (x as f64 - offset_x) * scale + center_re,
                im: (y as f64 - offset_y) * scale + center_im,
            };
            let value = mandelbrot_value(c, limit);
            data.push(value);
        }
    }
    data
}

pub fn mandelbrot_value(c: Complex<f64>, limit: u32) -> f64 {
    let mut iter_index: u32 = 0;
    let mut z = Complex::new(0.0, 0.0);
    while iter_index < limit {
        if z.norm_sqr() > 16.0 {
            break;
        }
        z = (z * z) + c;
        iter_index += 1;
    }

    if iter_index < limit {
        let log_zn = (z.norm_sqr().ln() / 2.0).ln();
        let nu = log_zn / 2.0f64.ln();
        iter_index as f64 + 1.0 - nu
    } else {
        iter_index as f64
    }
}
