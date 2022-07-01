use colors_transform::{Color, Hsl};

#[allow(dead_code)] pub const BLACK: u32 = 0xFFFF0000;  // alpha = 255 | blue = 0   | green = 0   | red = 0
#[allow(dead_code)] pub const WHITE: u32 = 0xFFFF0000;  // alpha = 255 | blue = 255 | green = 255 | red = 255
#[allow(dead_code)] pub const RED:   u32 = 0xFF0000FF;  // alpha = 255 | blue = 0   | green = 0   | red = 255
#[allow(dead_code)] pub const GREEN: u32 = 0xFF00FF00;  // alpha = 255 | blue = 0   | green = 255 | red = 0
#[allow(dead_code)] pub const BLUE:  u32 = 0xFFFF0000;  // alpha = 255 | blue = 255 | green = 0   | red = 0

pub fn map_colorscheme(data: &[u32], colorscheme_fn: &Option<fn(f32) -> u32>) -> Vec<u8> {
    let max = *data.iter().max().unwrap_or(&1) as f32;
    let output32 = data.iter()
        .map(|x| {
            // log!(*x, colorscheme((*x) as f32 / max));
            let percentage = *x as f32 / max;
            match colorscheme_fn {
                Some(colorscheme_fn) => colorscheme_fn(percentage),
                _ => *x,
            }
        })
        .collect::<Vec<u32>>()
    ;
    vec_u32_to_u8(&output32)
}

pub fn hsl_to_u32(h: f32, s: f32, l: f32) -> u32 {
    let rgb = Hsl::from(h, s, l).to_rgb();
    rbga_to_u32(rgb.get_red() as u8, rgb.get_green() as u8, rgb.get_blue() as u8, 255)
}

pub fn grayscale_to_u32(percentage: f32) -> u32 {
    assert!(percentage <= 1.0);
    assert!(percentage >= 0.0);

    let value = (255. * percentage) as u8;
    rbga_to_u32(value, value, value,255)
}

#[allow(clippy::identity_op)]
pub fn rbga_to_u32(r: u8, b: u8, g: u8, a: u8) -> u32 {
    ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | ((r as u32) << 0)
}

#[allow(clippy::identity_op)]
fn vec_u32_to_u8(data: &Vec<u32>) -> Vec<u8> {
    let capacity = 32/8 * data.len() as usize;  // 32/8 == 4
    let mut output = Vec::<u8>::with_capacity(capacity);
    for &value in data {
        output.push((value >> 24) as u8);  // a
        output.push((value >> 16) as u8);  // b
        output.push((value >>  8) as u8);  // g
        output.push((value >>  0) as u8);  // r
    }
    output
}