use colors_transform::{Color, Hsl};
use contracts::requires;

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


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
pub fn grayscale_to_u32(percentage: f32) -> u32 {
    let value = (255. * percentage) as u8;
    rbga_to_u32(value, value, value,255)
}

#[allow(clippy::identity_op)]
pub fn rbga_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | ((a as u32) << 0)
}


#[cfg(test)]
mod tests {
    use crate::services::colors::*;

    use super::*;

    mod rbga_to_u32 {
        use super::*;

        #[test]
        fn red() {
            let result = rbga_to_u32(255 as u8, 0 as u8, 0 as u8, 255 as u8);
            assert_eq!(result, RED, "{:X} != {:X}", result, RED);
        }
        #[test]
        fn green() {
            let result = rbga_to_u32(0 as u8, 255 as u8, 0 as u8, 255 as u8);
            assert_eq!(result, GREEN, "{:X} != {:X}", result, GREEN);
        }
        #[test]
        fn blue() {
            let result = rbga_to_u32(0 as u8, 0 as u8, 255 as u8, 255 as u8);
            assert_eq!(result, BLUE, "{:X} != {:X}", result, BLUE);
        }
        #[test]
        fn black() {
            let result = rbga_to_u32(0 as u8, 0 as u8, 0 as u8, 255 as u8);
            assert_eq!(result, BLACK, "{:X} != {:X}", result, BLACK);
        }
        #[test]
        fn white() {
            let result = rbga_to_u32(255 as u8, 255 as u8, 255 as u8, 255 as u8);
            assert_eq!(result, WHITE, "{:X} != {:X}", result, WHITE);
        }
    }

    mod hsl_to_u32 {
        use super::*;

        #[test]
        fn test_hsl_to_u32() {
            let colormap = [
                (  0.,   0.,   0.,  BLACK),
                (  0.,   0., 100.,  WHITE),
                (  0.,   0.,  50.,  0x7F7F7FFF),  // GRAY
                (  0., 100.,  50.,  RED),
                (120., 100.,  50.,  GREEN),
                (240., 100.,  50.,  BLUE),
            ];
            for (h, s, l, hex) in colormap {
                let result = hsl_to_u32(h as f32, s as f32, l as f32);
                assert_eq!(result, hex,
                           "hsl_to_u32({}, {}, {}) | 0x{:08X} != 0x{:08X} RGBA", h, s, l, result, hex);
            }
        }
    }

    mod grayscale_to_u32 {
        use super::*;

        #[test]
        fn test_grayscale_to_u32() {
            let colormap = [
                (0.0,   BLACK),
                (0.251, 0x404040FF),
                (0.50,  0x7F7F7FFF),
                (0.505, 0x808080FF),
                (0.75,  0xBFBFBFFF),
                (0.755, 0xC0C0C0FF),
                (1.0,   WHITE),
            ];
            for (percentage, hex) in colormap {
                let result = grayscale_to_u32(percentage);
                assert_eq!(result, hex,
                           "grayscale_to_u32({}) | 0x{:08X} != 0x{:08X} RGBA", percentage, result, hex);
            }
        }
    }
}