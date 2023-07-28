use num_complex::Complex;

pub struct Rectangle {
    pub center: Complex<f32>,
    pub left:   f32,
    pub right:  f32,
    pub top:    f32,
    pub bottom: f32,
}

impl Rectangle {
    pub fn new(center: Complex<f32>, scale: f32, width: u32, height: u32) -> Self {
        let aspect_ratio = height as f32 / width as f32;
        Self {
            center,
            left:   center.re - scale,
            right:  center.re + scale,
            top:    center.im - scale * aspect_ratio,
            bottom: center.im + scale * aspect_ratio,
        }
    }
    pub fn width(&self)  -> f32 { self.right  - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top  }
}
