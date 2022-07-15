use contracts::requires;

use crate::services::color_convert::{grayscale_to_u32, hsl_to_u32, rbga_to_u32};
use crate::services::colors::*;

pub enum ColorScheme {
    HSL,
    Grayscale,
    Green,
    UltraFractal,
}
impl ColorScheme {
    #[allow(dead_code)]
    #[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
    #[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
    pub fn color(&self, percentage: f32) -> u32 {
        match self {
            ColorScheme::HSL          => colorscheme_hsl(percentage),
            ColorScheme::Grayscale    => colorscheme_grayscale(percentage),
            ColorScheme::Green        => colorscheme_green(percentage),
            ColorScheme::UltraFractal => colorscheme_ultrafractal(percentage),
        }
    }
}


#[allow(dead_code)]
#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn colorscheme_hsl(percentage: f32) -> u32 {
    hsl_to_u32(360. * percentage, 100., 50.)
}

#[allow(dead_code)]
#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn colorscheme_grayscale(percentage: f32) -> u32 {
    if      percentage == 0.0 { WHITE }
    else if percentage == 1.0 { BLACK }
    else {  grayscale_to_u32(percentage) }
}

#[allow(dead_code)]
#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn colorscheme_green(percentage: f32) -> u32 {
    // Source: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
    let color: u8 = (255. * percentage) as u8;
    if percentage > 0.5 {
        rbga_to_u32(color, 255, color, 255)
    } else {
        rbga_to_u32(0,color, 0, 255)
    }
}

#[allow(dead_code)]
#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn colorscheme_ultrafractal(percentage: f32) -> u32 {
    // Source: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
    let pallete = [
        (0, 0, 0),
        (66, 30, 15),
        (25, 7, 26),
        (9, 1, 47),
        (4, 4, 73),
        (0, 7, 100),
        (12, 44, 138),
        (24, 82, 177),
        (57, 125, 209),
        (134, 181, 229),
        (211, 236, 248),
        (241, 233, 191),
        (248, 201, 95),
        (255, 170, 0),
        (204, 128, 0),
        (153, 87, 0),
        (106, 52, 3),
        (0, 0, 0),
    ];
    let index = (percentage * (pallete.len()-1) as f32) as usize;
    // let index =
    //     if      percentage == 0.0 { 0 }
    //     else if percentage == 1.0 { pallete.len()-1 }
    //     else                      { (1. + percentage * (pallete.len() - 2) as f32) as usize }
    // ;
    let color = pallete[index];
    rbga_to_u32(color.0,color.1, color.2, 255)
}
