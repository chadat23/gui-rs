#[derive(Clone, Copy, Debug)]
pub struct GUILength {
    pub length: f64,
}

impl Default for GUILength {
    fn default() -> Self {
        Self { length: 0. }
    }
}

impl PartialEq for GUILength {
    fn eq(&self, other: &GUILength) -> bool {
        let decimal_places = 8f64;
        let multiplyer = (10f64).powf(decimal_places);

        (self.length * multiplyer).round() == (other.length * multiplyer).round()
    }
}

impl GUILength {
    pub fn negative(&self) -> Self {
        Self {
            length: -self.length,
        }
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_physical_length(&self, scale: &f64) -> f64 {
        (self.length * scale).round()
    }

    pub fn from_pixels(pixels: f64) -> Self {
        Self { length: pixels }
    }

    pub fn from_physical_pixels(pixels: f64, scale: &f64) -> Self {
        Self {
            length: pixels / scale,
        }
    }

    pub fn add(&self, other: &GUILength) -> Self {
        Self {
            length: self.length + other.length,
        }
    }

    pub fn subtract(&self, other: &GUILength) -> Self {
        Self {
            length: self.length - other.length,
        }
    }

    pub fn multiply(&self, other: &GUILength) -> Self {
        Self {
            length: self.length * other.length,
        }
    }

    pub fn devide_by(&self, other: &GUILength) -> Self {
        Self {
            length: self.length / other.length,
        }
    }
}

/// Represents an area (width and height)
#[derive(Copy, Clone, Debug)]
pub struct GUISize {
    pub width: GUILength,
    pub height: GUILength,
}

impl Default for GUISize {
    fn default() -> GUISize {
        GUISize::from_pixels(500., 500.)
    }
}

impl GUISize {
    pub fn from_lengths(width: GUILength, height: GUILength) -> Self {
        Self { width, height }
    }

    pub fn from_pixels(width: f64, height: f64) -> Self {
        Self {
            width: GUILength::from_pixels(width),
            height: GUILength::from_pixels(height),
        }
    }

    pub fn from_physical_pixels(width: f64, height: f64, scale: &f64) -> Self {
        Self {
            width: GUILength::from_physical_pixels(width, &scale),
            height: GUILength::from_physical_pixels(height, &scale),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GUIPosition {
    pub x: GUILength,
    pub y: GUILength,
}

impl PartialEq for GUIPosition {
    fn eq(&self, other: &GUIPosition) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl GUIPosition {
    pub fn from_pixels(x: f64, y: f64) -> Self {
        Self {
            x: GUILength::from_pixels(x),
            y: GUILength::from_pixels(y),
        }
    }

    pub fn from_lengths(x: GUILength, y: GUILength) -> Self {
        Self { x, y }
    }

    pub fn from_physical_pixels(x: f64, y: f64, scale: &f64) -> Self {
        Self {
            x: GUILength::from_physical_pixels(x, &scale),
            y: GUILength::from_physical_pixels(y, &scale),
        }
    }
}
