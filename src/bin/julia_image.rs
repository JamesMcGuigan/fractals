/// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
/// #![cfg_attr(debug_assertions, allow(dead_code))]
extern crate fractals;
extern crate yew;

use image::{ImageFormat, save_buffer_with_format};
use num_complex::Complex;
use structopt::StructOpt;
// use wasm_timer::Instant;

use fractals::mathematics::julia_set::julia_set;
use fractals::services::colorschemes::ColorScheme;
use fractals::services::vectors::{map_colorscheme, vec_u32_to_u8, vec_u8_rgba_to_rgb};


// DOCS: https://docs.rs/structopt/latest/structopt/
// DOCS: https://www.youtube.com/watch?v=Des3zZuTbhk&ab_channel=CodingTech
// TODO: Add XY coords
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long, default_value="fractal.png")]
    output: String,
    #[structopt(short, long, default_value="0.5+0.5i")]
    c: Complex<f32>,
    #[structopt(short, long, default_value="2.0")]
    radius: f32,
    #[structopt(short, long, default_value="64")]
    limit: u32,
    #[structopt(short, long, default_value="640")]
    width: u32,
    #[structopt(short, long, default_value="480")]
    height: u32,
}


fn main() {
    // BUG: Instant::now() causes panic in browser WASM ???
    // let time_now = Instant::now();

    let opt = Opt::from_args();
    fractal_to_png(&opt);

    // let time_taken = time_now.elapsed();
    // println!("wrote: {} ({:.1?})", opt.output, time_taken);
    println!("wrote: {}", opt.output);
}

fn fractal_to_png(opt: &Opt) {
    let data_julia: Vec<u32> = julia_set(opt.c, opt.width, opt.height, opt.radius, opt.limit);
    let data_color: Vec<u32> = map_colorscheme(&data_julia, ColorScheme::Green);
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