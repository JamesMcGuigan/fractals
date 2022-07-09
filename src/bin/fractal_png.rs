/// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
/// #![cfg_attr(debug_assertions, allow(dead_code))]
extern crate fractals;
extern crate yew;

use image::save_buffer_with_format;
use num_complex::Complex;

use fractals::mathematics::julia_set::julia_set;
use fractals::services::colorschemes::colorscheme_hsl;
use fractals::services::vectors::{map_colorscheme, vec_u32_to_u8, vec_u8_rgba_to_rgb};

fn main() {
    let time_now= std::time::Instant::now();
    fractal_to_png();
    let time_taken = time_now.elapsed();
    println!("wrote: fractal.png ({:.1?})", time_taken);
}

fn fractal_to_png() {
    let width  = 640;
    let height = 480;
    let real   = 0.5;
    let imag   = 0.5;
    let radius = 2.0;
    let limit  = 64;

    let c = Complex::<f32>::new(real, imag);
    let data_julia: Vec<u32> = julia_set(c, width, height, radius, limit);
    let data_color: Vec<u32> = map_colorscheme(&data_julia, &Some(colorscheme_hsl));
    let data_color_u8_rgba: Vec<u8> = vec_u32_to_u8(&data_color);
    let data_color_u8_rgb:  Vec<u8> = vec_u8_rgba_to_rgb(&data_color_u8_rgba);

    // DOCS: https://stackoverflow.com/questions/65066172/convert-vecu8-rgb-data-to-imagebuffer-rust
    save_buffer_with_format(
        "fractal.png",
        &data_color_u8_rgb, width, height,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    ).unwrap();
}