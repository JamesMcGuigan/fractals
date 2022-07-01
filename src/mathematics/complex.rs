use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

impl Complex {
    pub fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imag * self.imag);
        let imag = 2.0 * self.real * self.imag;
        Complex { real, imag }
    }

    pub fn norm(self) -> f32 {
        (self.real * self.real) + (self.imag * self.imag)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}
