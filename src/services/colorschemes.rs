use contracts::requires;

use crate::services::color_convert::{grayscale_to_u32, hsl_to_u32};
use crate::services::colors::*;

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
