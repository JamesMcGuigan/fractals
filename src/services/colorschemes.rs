// use std::fmt;
use contracts::requires;
use enum_iterator::Sequence;
use strum_macros::{Display, EnumString};

use crate::services::color_convert::{grayscale_to_u32, hsl_to_u32, rbga_to_u32};
use crate::services::colors::*;

#[derive(Copy, Clone, Debug, PartialEq, Display, EnumString, Sequence)]
pub enum ColorScheme {
    HSL,
    Grayscale,
    Green,
    Ultra,
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
            ColorScheme::Ultra        => colorscheme_ultra(percentage),
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
pub fn colorscheme_ultra(percentage: f32) -> u32 {
    // Source: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
    let pallet = vec![
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
    colorscheme_pallete(percentage, &pallet)
}


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn colorscheme_pallete(percentage: f32, pallet: &Vec<(u8, u8, u8)>) -> u32 {
    let index: usize =
        if      percentage == 0.0 { 0 }
        else if percentage == 1.0 { pallet.len() - 1 }
        else    { (1. + (percentage * (pallet.len() - 2) as f32)) as usize }
    ;
    let color = pallet[index];
    rbga_to_u32(color.0,color.1, color.2, 255)
}