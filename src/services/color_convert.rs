use colors_transform::{Color, Hsl};
use contracts::requires;

/// Convert hsl to rgba u32
/// ```
/// # use fractals::services::color_convert::hsl_to_u32;
/// # use fractals::services::colors::*;
/// for (h, s, l, hex) in [
///     (  0.,   0.,   0.,  BLACK),
///     (  0.,   0., 100.,  WHITE),
///     (  0.,   0.,  50.,  0x7F7F7FFF),  // GRAY
///     (  0., 100.,  50.,  RED),
///     (120., 100.,  50.,  GREEN),
///     (240., 100.,  50.,  BLUE),
/// ] {
///     let result = hsl_to_u32(h as f32, s as f32, l as f32);
///     assert_eq!(result, hex, "hsl_to_u32({}, {}, {}) | 0x{:08X} != 0x{:08X} RGBA", h, s, l, result, hex);
/// }
/// ```
#[requires(h >=   0.0, "360. >= h >= 0.")]
#[requires(h <= 360.0, "360. >= h >= 0.")]
#[requires(s >=   0.0, "100. >= s >= 0.")]
#[requires(s <= 100.0, "100. >= s >= 0.")]
#[requires(l >=   0.0, "100. >= l >= 0.")]
#[requires(l <= 100.0, "100. >= l >= 0.")]
pub fn hsl_to_u32(h: f32, s: f32, l: f32) -> u32 {
    let rgb = Hsl::from(h, s, l).to_rgb();
    rbga_to_u32(rgb.get_red() as u8, rgb.get_green() as u8, rgb.get_blue() as u8, 255)
}


/// Convert a percentage value to grayscale rgba u32
/// ```
/// # use fractals::services::color_convert::grayscale_to_u32;
/// assert_eq!(grayscale_to_u32(0.0),  0x000000FF);
/// assert_eq!(grayscale_to_u32(0.25), 0x3F3F3FFF);
/// assert_eq!(grayscale_to_u32(0.5),  0x7F7F7FFF);
/// assert_eq!(grayscale_to_u32(0.75), 0xBFBFBFFF);
/// assert_eq!(grayscale_to_u32(1.0),  0xFFFFFFFF);
/// ```
#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn grayscale_to_u32(percentage: f32) -> u32 {
    let value = (255. * percentage) as u8;
    rbga_to_u32(value, value, value,255)
}

/// Convert rgba u8's into u32 hex
/// ```
/// # use fractals::services::color_convert::rbga_to_u32;
/// # use fractals::services::colors::*;
/// assert_eq!(rbga_to_u32(255,0,0,255),     RED  );
/// assert_eq!(rbga_to_u32(0,255,0,255),     GREEN);
/// assert_eq!(rbga_to_u32(0,0,255,255),     BLUE );
/// assert_eq!(rbga_to_u32(255,255,255,255), WHITE);
/// assert_eq!(rbga_to_u32(0,0,0,255),       BLACK);
/// ```
#[allow(clippy::identity_op)]
pub fn rbga_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | ((a as u32) << 0)
}
