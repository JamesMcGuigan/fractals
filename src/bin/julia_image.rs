use std::time::Instant;

use image::{ImageFormat, save_buffer_with_format};
use num_complex::Complex;
use structopt::StructOpt;

use fractals::mathematics::julia_set::julia_set;
use fractals::mathematics::mandelbrot::mandelbrot_set;
use fractals::services::colorschemes::ColorScheme;
use fractals::services::vectors::{map_colorscheme, vec_u32_to_u8, vec_u8_rgba_to_rgb};

// DOCS: https://docs.rs/structopt/latest/structopt/
// DOCS: https://www.youtube.com/watch?v=Des3zZuTbhk&ab_channel=CodingTech
// TODO: Add XY coords
#[derive(Debug, StructOpt)]
#[structopt(name = "fractal_image", about = "Generate a fractal image file")]
struct Opt {
    #[structopt(short, long, default_value="fractal.png")]
    output: String,
    #[structopt(short, long, default_value="mandelbrot")]
    fractal_type: String,
    #[structopt(short, long, default_value="-1.0+0.0i")]
    c: Complex<f64>,
    #[structopt(short, long, default_value="0.0")]
    center_re: f64,
    #[structopt(short, long, default_value="0.0")]
    center_im: f64,
    #[structopt(short, long, default_value="2.0")]
    radius: f64,
    #[structopt(short, long, default_value="64")]
    limit: u32,
    #[structopt(short, long, default_value="640")]
    width: u32,
    #[structopt(short, long, default_value="480")]
    height: u32,
}


fn main() {
    // BUG: Instant::now() causes panic in browser WASM ???
    // BUGFIX: Move CLI into separate module
    let time_now = Instant::now();

    let opt = Opt::from_args();
    fractal_to_png(&opt);

    let time_taken = time_now.elapsed();
    println!("wrote: {} ({:.1?})", opt.output, time_taken);
}

fn fractal_to_png(opt: &Opt) {
    let data_fractal: Vec<f64> = match opt.fractal_type.to_lowercase().as_str() {
        "mandelbrot" => mandelbrot_set(opt.width, opt.height, opt.center_re, opt.center_im, opt.radius, opt.limit),
        _ => julia_set(opt.c, opt.width, opt.height, opt.center_re, opt.center_im, opt.radius, opt.limit),
    };
    let data_color: Vec<u32> = map_colorscheme(&data_fractal, ColorScheme::Ultra);
    let data_color_u8_rgba: Vec<u8> = vec_u32_to_u8(&data_color);
    let data_color_u8_rgb:  Vec<u8> = vec_u8_rgba_to_rgb(&data_color_u8_rgba);
    let format = ImageFormat::from_path(&opt.output).unwrap();

    // DOCS: https://stackoverflow.com/questions/65066172/convert-vecu8-rgb-data-to-imagebuffer-rust
    save_buffer_with_format(
        opt.output.clone(),
        &data_color_u8_rgb, opt.width, opt.height,
        image::ColorType::Rgb8,
        format,
    ).unwrap();
}