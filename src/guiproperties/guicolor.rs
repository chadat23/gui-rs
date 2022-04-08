/// Represents a color and opacity.
#[derive(Clone, Copy)]
pub struct GUIColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Default for GUIColor {
    fn default() -> Self {
        Self {
            r: 0.4,
            g: 0.4,
            b: 0.4,
            a: 1.,
        }
    }
}

impl GUIColor {
    /// Creates an GUIColor from u8 red, green, blue, and alpha values.
    pub fn from_rgba_u8u8u8u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f64 / 255.,
            g: g as f64 / 255.,
            b: b as f64 / 255.,
            a: a as f64 / 255.,
        }
    }
}
