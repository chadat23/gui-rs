use crate::guiprocessing::vertices::Vertex;
use crate::guiproperties::guiposition::{GUILength, GUISize};
use crate::guiproperties::guitraits::Widget;
use crate::guiproperties::GUIColor;

// #[derive(Clone, Copy)]
pub struct GUIWindow {
    // /// The tile of the window.
    pub title: &'static str,
    /// The size of the window.
    pub size: GUISize,
    /// The minimum size of the window.
    pub min_size: GUISize,
    // /// The minimum size of the window.
    // pub max_size: GUISize,
    // /// Whether or not the window is resizable.
    // pub resizable: bool,
    // /// Whether or not the window is always on top of other windows.
    // pub always_on_top: bool,
    // /// The window's icon
    // pub window_icon: Option<GUIIcon>,
    // /// The window's IME position
    // pub ime_position: Option<GUIPosition>,
    // /// The background color for the window.
    pub background_color: GUIColor,
    // /// A list of child widgets.
    // pub children: Vec<Box<dyn Family>>,
    // /// The scale that converts between the devices logical and physical pixels.
    // pub logical_scale: Option<f64>,
    // /// The human readable name of the window
    // pub name: &'static str,
    pub id: i128,
}

impl Widget for GUIWindow {
    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        indice_offset: u16,
    ) -> (Vec<Vertex>, Vec<u16>) {
        (Vec::new(), Vec::new())
    }

    fn get_size(&self) -> &GUISize {
        &self.size
    }
}

impl GUIWindow {
    pub fn new(id: i128) -> Self {
        Self {
            title: "Form1",
            size: GUISize {
                width: GUILength::from_pixels(500.),
                height: GUILength::from_pixels(500.),
            },
            min_size: GUISize {
                width: GUILength::from_pixels(100.),
                height: GUILength::from_pixels(100.),
            },
            // max_size: GUISize {
            //     width: GUILength::from_pixels(800.),
            //     height: GUILength::from_pixels(800.),
            // },
            // resizable: true,
            // always_on_top: false,
            // window_icon: None,
            // ime_position: None,
            background_color: GUIColor {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 1.0,
            },
            // children: Vec::new(),
            // logical_scale: None,
            // name: DEFAULT_WINDOW_NAME,
            id,
        }
    }
}
