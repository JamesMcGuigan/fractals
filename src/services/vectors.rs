use crate::services::colorschemes::ColorScheme;

#[allow(clippy::identity_op)]
pub fn vec_u32_to_u8(data: &Vec<u32>) -> Vec<u8> {
    // TODO: https://stackoverflow.com/questions/72631065/how-to-convert-a-u32-array-to-a-u8-array-in-place
    // TODO: https://stackoverflow.com/questions/29037033/how-to-slice-a-large-veci32-as-u8
    let capacity = 32/8 * data.len() as usize;  // 32/8 == 4
    let mut output = Vec::<u8>::with_capacity(capacity);
    for &value in data {
        output.push((value >> 24) as u8);  // r
        output.push((value >> 16) as u8);  // g
        output.push((value >>  8) as u8);  // b
        output.push((value >>  0) as u8);  // a
    }
    output
}

// Remove alpha channel from RGBA stream - required for save_buffer_with_format()
pub fn vec_u8_rgba_to_rgb(rgba: &[u8]) -> Vec<u8> {
    rgba.iter()
        .enumerate()
        .filter(|(i,_x)| i % 4 != 3)
        .map(|(_i,x)| *x)
        .collect::<Vec<u8>>()
}


pub fn map_colorscheme(data: &[u32], colorscheme: ColorScheme) -> Vec<u32> {
    let max = *data.iter().max().unwrap_or(&1) as f32;
    let output32 = data.iter()
        .map(|x| {
            // log!(*x, colorscheme((*x) as f32 / max));
            let percentage = *x as f32 / max;
            colorscheme.color(percentage)
        })
        .collect::<Vec<u32>>();
    output32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_u32_to_u8() {
        let input:    Vec<u32> = vec![ 0x12345678 ];
        let expected: Vec<u8>  = vec![ 0x12, 0x34, 0x56, 0x78 ];
        let output = vec_u32_to_u8(&input);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_vec_u8_rgba_to_rgb() {
        let input:    Vec<u8> = vec![ 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78 ];
        let expected: Vec<u8> = vec![ 0x12, 0x34, 0x56,       0x12, 0x34, 0x56       ];
        let output = vec_u8_rgba_to_rgb(&input);
        assert_eq!(expected, output);
    }
}