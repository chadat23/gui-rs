use std::collections::HashMap;

// use super::super::guiproperties::Widget;
use super::{GUIButton, GUIWindow};
use crate::guiproperties::guiposition::GUILength;
use crate::guiproperties::guitraits::{Parent, Widget};

pub struct GUIBase {
    pub base_window: u128,
    pub windows: HashMap<u128, GWindow>,
    pub widgets: HashMap<u128, GWidget>,
    /// The scale that converts between the devices logical and physical pixels.
    pub logical_scale: Option<f64>,
}

impl GUIBase {
    pub fn new() -> Self {
        Self {
            base_window: 0,
            windows: HashMap::new(),
            widgets: HashMap::new(),
            logical_scale: None,
        }
    }

    pub fn get_base_window(&self) -> &GUIWindow {
        &self.windows.get(&self.base_window).unwrap().window
    }

    pub fn get_base_window_mut(&mut self) -> &mut GUIWindow {
        &mut self.windows.get_mut(&self.base_window).unwrap().window
    }

    pub fn add_window(&mut self, window: GUIWindow) -> u128 {
        if self.windows.len() == 0 {
            self.base_window = *window.get_id();
        }

        let window_id = window.id;

        let gwindow = GWindow {
            window,
            children: Vec::new(),
        };
        self.windows.insert(window_id, gwindow);

        window_id
    }

    pub fn get_widget(&self, id: u128) -> &Box<dyn Widget> {
        &self.widgets.get(&id).unwrap().widget
    }

    pub fn add_child_to_parent<T: 'static + Widget>(&mut self, child: T, parent_id: u128) -> u128 {
        let child_id = *child.get_id();

        let gwidget = GWidget {
            widget: Box::new(child),
            parent: parent_id,
            children: Vec::new(),
        };

        if self.windows.contains_key(&parent_id) {
            let window = self.windows.get_mut(&parent_id).unwrap();
            window.children.push(child_id);
        } else if self.widgets.contains_key(&parent_id) {
            let widget = self.widgets.get_mut(&parent_id).unwrap();
            widget.children.push(child_id);
        } else {
            panic!("oops")
        }

        self.widgets.insert(child_id, gwidget);

        child_id
    }
}

// #[derive(Clone, Copy)]
pub struct GWindow {
    pub window: GUIWindow,
    pub children: Vec<u128>,
}

impl GWindow {
    pub fn get_window(&self) -> &GUIWindow {
        &self.window
    }

    pub fn get_window_mut(&mut self) -> &mut GUIWindow {
        // self.window.clone()
        &mut self.window
    }

    pub fn get_child_ids(&self) -> &Vec<u128> {
        &self.children
    }
}

pub struct GWidget {
    widget: Box<dyn Widget>,
    parent: u128,
    children: Vec<u128>,
}

impl GWidget {
    pub fn get_widget(&self) -> &Box<dyn Widget> {
        &self.widget
    }

    pub fn get_widget_mut(&mut self) -> &mut Box<dyn Widget> {
        &mut self.widget
    }

    pub fn get_parent_id(&self) -> &u128 {
        &self.parent
    }

    pub fn get_child_ids(&self) -> &Vec<u128> {
        &self.children
    }
}
