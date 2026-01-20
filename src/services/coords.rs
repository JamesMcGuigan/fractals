use num_complex::Complex;

#[allow(dead_code)]
pub fn coords_from_pixels(x: u32, y: u32, width: u32, height: u32, radius: f64) -> Complex<f64> {
    let min_side = std::cmp::min(width, height) as f64;
    let scale    = 2. * radius / min_side;
    let offset_x = width  as f64 / 2.;
    let offset_y = height as f64 / 2.;

    Complex {
        re: (y as f64 - offset_y) * scale,
        im: (x as f64 - offset_x) * scale,
    }
}