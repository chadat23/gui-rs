use core::num;
use std::any::Any;

use uuid::Uuid;

use super::widget_utils;
use super::widget_utils::arcs;
use crate::guiprocessing::vertices::{Vertex, Triangles};
use crate::guiproperties::guiposition::{GUILength, GUIPosition, GUISize};
use crate::guiproperties::guitraits::Widget;
use crate::guiproperties::GUIColor;

pub struct GUIButton {
    /// The tile of the button.
    pub text: &'static str,
    /// The size of the button.
    pub size: GUISize,
    /// The location of the button.
    pub position: GUIPosition,
    /// Radius of the button corners.
    pub radius: GUILength,
    /// The background color for the button.
    pub background_color: GUIColor,
    /// The human readable name of the button
    // pub name: &'static str,
    pub id: u128,
}

impl Widget for GUIButton {
    fn get_size(&self) -> &GUISize {
        &self.size
    }

    fn get_id(&self) -> &u128 {
        &self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        indice_offset: u16,
    ) -> (Vec<Vertex>, Vec<u16>, Triangles) {
        const FASCET_COUNT: usize = 7;
        let mut top_left_radius = arcs::make_top_left_arc(self.radius, FASCET_COUNT);
        top_left_radius = widget_utils::translate(
            top_left_radius,
            &self.radius.add(&self.position.x),
            &self.radius.add(&self.position.y),
        );

        let top_right_radius = arcs::make_top_right_arc(self.radius, FASCET_COUNT);
        let top_right_radius = widget_utils::translate(
            top_right_radius,
            &self.size.width.subtract(&self.radius).add(&self.position.x),
            &self.radius.add(&self.position.y),
        );

        let bottom_left_radius = arcs::make_bottom_left_arc(self.radius, FASCET_COUNT);
        let bottom_left_radius = widget_utils::translate(
            bottom_left_radius,
            &self.radius.add(&self.position.x),
            &self
                .size
                .height
                .subtract(&self.radius)
                .add(&self.position.y),
        );

        let bottom_right_radius = arcs::make_bottom_right_arc(self.radius, FASCET_COUNT);
        let bottom_right_radius = widget_utils::translate(
            bottom_right_radius,
            &self.size.width.subtract(&self.radius).add(&self.position.x),
            &self
                .size
                .height
                .subtract(&self.radius)
                .add(&self.position.y),
        );

        top_left_radius.extend(bottom_left_radius);
        top_left_radius.extend(bottom_right_radius);
        top_left_radius.extend(top_right_radius);

        let mut vertices = Vec::with_capacity(top_left_radius.len());
        for position in top_left_radius.iter() {
            vertices.push(Vertex {
                position: [
                    (position.x.get_length() / parent_size.width.get_length() - 1.) as f32,
                    (-position.y.get_length() / parent_size.height.get_length() + 1.) as f32,
                    0.,
                ],
                color: [
                    self.background_color.r as f32,
                    self.background_color.g as f32,
                    self.background_color.b as f32,
                ],
                // id: self.id as u128,
            });
        }
        let number_of_triangles = vertices.len() - 2;
        let mut indices = Vec::with_capacity(number_of_triangles * 3);
        let mut widget_id = Vec::with_capacity(number_of_triangles);
        let mut triangles = Vec::with_capacity(top_left_radius.len() - 2);
        for i in 0..number_of_triangles {
            indices.push(indice_offset + 0);
            indices.push(indice_offset + (i + 1) as u16);
            indices.push(indice_offset + (i + 2) as u16);
            widget_id.push(self.id);
            triangles.push([top_left_radius.get(0).unwrap().clone(), top_left_radius.get(i + 1).unwrap().clone(), top_left_radius.get(i + 2).unwrap().clone()]);
            
        }

        let triangles = Triangles {
            triangles,
            widget_id,
        };

        (vertices, indices, triangles)
    }

    // fn set_position_from_pixels(&mut self, x: f64, y: f64) {
    //     self.position = GUIPosition::from_pixels(x, y);
    // }

    // fn set_position_from_lengths(&mut self, x: GUILength, y: GUILength) {
    //     self.position = GUIPosition::from_lengths(x, y);
    // }

    // fn set_position_from_position(&mut self, position: GUIPosition) {
    //     self.position = position;
    // }
}
impl Default for GUIButton {
    /// Returns a button with all of the default values.
    fn default() -> Self {
        Self {
            text: "Button",
            size: GUISize {
                width: GUILength::from_pixels(200.),
                height: GUILength::from_pixels(100.),
            },
            position: GUIPosition::from_pixels(0., 0.),
            radius: GUILength::from_pixels(25.),
            background_color: GUIColor {
                r: 0.7,
                g: 0.1,
                b: 0.4,
                a: 1.0,
            },
            id: Uuid::new_v4().as_u128(),
        }
    }
}
