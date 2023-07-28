use num_complex::Complex;
use crate::components::v2::rectangle::Rectangle;

#[allow(dead_code)]
pub struct Camera {
    pixel_count:     (u32, u32),  // (1920, 1080) screen size px (larger is wider)
    pixel_resolution: u8,         // 1  square screen pixels per function pixel (larger is more granular)

    center: Complex<f32>,         // (0,0) fn position at screen centerpoint
    scale:  f32,                  // 2     fn distance from centerpoint to wide screen edge (smaller is zoomed in)
}

impl Camera {

    /// shape of domain grid to be calculated, which may be smaller than pixel grid
    #[allow(dead_code)]
    fn shape(&self) -> (u32,u32) {
        // // division by integer promotion == .ceil() as u32 (faster)
        // return (
        //     (self.pixel_count.0 + self.pixel_resolution - 1) / self.pixel_resolution,
        //     (self.pixel_count.1 + self.pixel_resolution - 1) / self.pixel_resolution,
        // )
        // // division via floating point conversion (slower)
        (
            (self.pixel_count.0 as f32 / self.pixel_resolution as f32).ceil() as u32,
            (self.pixel_count.1 as f32 / self.pixel_resolution as f32).ceil() as u32
        )
    }

    /// get 2D array of fn model coordinates currently in camera view
    #[allow(dead_code)]
    fn inputs(&self) -> Vec<Vec<Complex<f32>>> {
        let (width, height) = self.shape();
        let domain = Rectangle::new(self.center, self.scale, width, height);
        let domain_width  = domain.width()  / width  as f32;  // domain width  per grid square
        let domain_height = domain.height() / height as f32;  // domain height per grid square

        (0..width).map(|col| {
            (0..height).map(|row| {
                let x = domain.left + domain_width  * col as f32;
                let y = domain.top  + domain_height * row as f32;
                Complex::new(x, y)
            }).collect()
        }).collect()
    }

}