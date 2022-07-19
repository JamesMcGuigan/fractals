use contracts::{ensures, requires};

use crate::services::color_convert::{grayscale_to_u32, hsl_to_u32, rbga_to_u32};
use crate::services::colors::*;

/// Enum of valid fractal ColorSchemes
/// ```
/// # use fractals::services::colors::*;
/// # use fractals::services::colorschemes::ColorScheme;
/// assert_eq!( ColorScheme::Grayscale.color(1.0), BLACK );
/// assert_eq!( ColorScheme::Grayscale.color(0.0), WHITE );
/// assert_eq!( ColorScheme::Grayscale.to_string(), "Grayscale" );
/// assert_eq!( ColorScheme::Grayscale, ColorScheme::from("Grayscale") );
/// ```
/// ```should_panic
/// # use fractals::services::colorschemes::ColorScheme;
/// ColorScheme::from("InvalidColor");
/// ```
#[derive(
    Copy, Clone, Debug, PartialEq, Eq,
    enum_utils::FromStr,          // color.parse().unwrap() -> ColorScheme
    enum_utils::IterVariants,     // ColorScheme::iter()
    strum_macros::Display,        // color.to_string()
    // enum_iterator::Sequence,   // all::<ColorScheme>().collect::()
)]
pub enum ColorScheme {
    HSL,
    Grayscale,
    Green,
    Ultra,
}
impl ColorScheme {
    /// Return a u32 rgba color from the selected ColorScheme
    #[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
    #[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
    pub fn color(&self, percentage: f32) -> u32 {
        match self {
            ColorScheme::HSL       => colorscheme_hsl(percentage),
            ColorScheme::Grayscale => colorscheme_grayscale(percentage),
            ColorScheme::Green     => colorscheme_green(percentage),
            ColorScheme::Ultra     => colorscheme_ultra(percentage),
        }
    }

    /// Return string values of the enum
    pub fn values() -> Vec<String> {
        ColorScheme::iter()
            .map(|color| color.to_string())
            .collect()
    }
    /// Return string values of the enum, joined together as a single string
    pub fn values_join(sep: &str) -> String {
        // #![feature(iter_intersperse)] -> std::iter::Intersperse
        // let options: String = ColorScheme::values().into_iter().intersperse("|".to_string()).collect();
        itertools::intersperse(
            ColorScheme::values().into_iter(),
            sep.to_string()
        ).collect()
    }

    /// Create ColorScheme from &str representation
    /// ```
    /// # use fractals::services::colorschemes::ColorScheme;
    /// assert_eq!( ColorScheme::Ultra, ColorScheme::from_string(String::from("Ultra")) );
    /// assert_eq!( ColorScheme::Ultra, ColorScheme::from_string(ColorScheme::Ultra.to_string()) );
    /// ```
    pub fn from_string(name: String) -> ColorScheme {
        ColorScheme::from(name.as_str())
    }
    /// Create ColorScheme from &str representation
    /// ```
    /// # use fractals::services::colorschemes::ColorScheme;
    /// assert_eq!( ColorScheme::Ultra, ColorScheme::from("Ultra") );
    /// assert_eq!( ColorScheme::Ultra, ColorScheme::from(&ColorScheme::Ultra.to_string()) );
    /// ```
    #[ensures(ColorScheme::values().iter().any(|value| value == name), "ColorScheme::from()")]
    pub fn from(name: &str) -> ColorScheme {
        let values = ColorScheme::values_join("|");
        name.parse()
            .unwrap_or_else(|_| panic!("ColorScheme::from({}) not in ({})", name, values))
    }
}


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
fn colorscheme_hsl(percentage: f32) -> u32 {
    hsl_to_u32(360. * percentage, 100., 50.)
}


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
fn colorscheme_grayscale(percentage: f32) -> u32 {
    if      percentage == 0.0 { WHITE }
    else if percentage == 1.0 { BLACK }
    else {  grayscale_to_u32(percentage) }
}


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
fn colorscheme_green(percentage: f32) -> u32 {
    // Source: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
    let color: u8 = (255. * percentage) as u8;
    if percentage > 0.5 {
        rbga_to_u32(color, 255, color, 255)
    } else {
        rbga_to_u32(0,color, 0, 255)
    }
}


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
fn colorscheme_ultra(percentage: f32) -> u32 {
    // Source: https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
    let pallet = vec![
        (0, 0, 0),
        // (66, 30, 15),  // Nasty brown color
        (25, 7, 26),
        (9, 1, 47),
        (4, 4, 73),
        (0, 7, 100),
        (12, 44, 138),
        (24, 82, 177),
        (57, 125, 209),
        (134, 181, 229),
        (211, 236, 248),
        (241, 233, 191),
        (248, 201, 95),
        (255, 170, 0),
        (204, 128, 0),
        (153, 87, 0),
        (106, 52, 3),
        (0, 0, 0),
    ];
    colorscheme_palette(percentage, &pallet)
}


#[requires(percentage >= 0.0, "1.0 >= percentage >= 0.0")]
#[requires(percentage <= 1.0, "1.0 >= percentage >= 0.0")]
fn colorscheme_palette(percentage: f32, pallet: &Vec<(u8, u8, u8)>) -> u32 {
    let index: usize =
        if      percentage == 0.0 { 0 }
        else if percentage == 1.0 { pallet.len() - 1 }
        else    { (1. + (percentage * (pallet.len() - 2) as f32)) as usize }
    ;
    let color = pallet[index];
    rbga_to_u32(color.0,color.1, color.2, 255)
}
