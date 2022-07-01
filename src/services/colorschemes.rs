use crate::services::colors::{BLACK, grayscale_to_u32, hsl_to_u32, WHITE};

#[allow(dead_code)]
pub fn colorscheme_hsl(percentage: f32) -> u32 {
    assert!(percentage <= 1.0);
    assert!(percentage >= 0.0);

    if      percentage == 0.0 { WHITE }
    else if percentage == 1.0 { BLACK }
    else {  hsl_to_u32(360. * percentage, 100., 50.) }
}

#[allow(dead_code)]
pub fn colorscheme_grayscale(percentage: f32) -> u32 {
    assert!(percentage <= 1.0);
    assert!(percentage >= 0.0);

    if      percentage == 0.0 { WHITE }
    else if percentage == 1.0 { BLACK }
    else {  grayscale_to_u32(percentage) }
}