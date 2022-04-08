use std::collections::HashMap;

// use super::super::guiproperties::Widget;
use crate::guiproperties::guitraits::Widget;
use super::GUIWindow;

pub struct GUIBase {
    pub windows: HashMap<i128, GWindow>,
    pub widgets: HashMap<i128, GWidget>,
    /// The scale that converts between the devices logical and physical pixels.
    pub logical_scale: Option<f64>,
}

impl GUIBase {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            widgets: HashMap::new(),
            logical_scale: None,
        }
    }

    pub fn next_window_id(&self) -> i128 {
        self.windows.len() as i128 - 1
    }

    pub fn add_window(&mut self, guiwindow: GUIWindow) {
        self.windows.insert(guiwindow.id, GWindow {
            window: guiwindow,
            children: Vec::new(),
        });
    }
}

// #[derive(Clone, Copy)]
pub struct GWindow {
    pub window: GUIWindow,
    pub children: Vec<i128>,
}

impl GWindow {
    pub fn get_window(&self) -> &GUIWindow {
        &self.window
    }

    pub fn get_window_mut(&mut self) -> &mut GUIWindow {
        // self.window.clone()
        &mut self.window
    }

    pub fn get_child_ids(&self) -> &Vec<i128> {
        &self.children
    }
}

pub struct GWidget {
    widget: Box<dyn Widget>,
    parent: i128,
    children: Vec<i128>,
}

impl GWidget {
    pub fn get_widget(&self) -> &Box<dyn Widget> {
        &self.widget
    }

    pub fn get_widget_mut(&mut self) -> &mut Box<dyn Widget> {
        &mut self.widget
    }

    pub fn get_parent_id(&self) -> &i128 {
        &self.parent
    }

    pub fn get_child_ids(&self) -> &Vec<i128> {
        &self.children
    }
}
