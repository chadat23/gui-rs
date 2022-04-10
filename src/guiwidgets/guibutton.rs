use std::any::Any;

use uuid::Uuid;

use super::widget_utils;
use super::widget_utils::arcs;
use crate::guiprocessing::vertices::{LogicalVertex, Polygon};
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
    ) -> (Vec<LogicalVertex>, Vec<u16>, Polygon) {
        const FASCET_COUNT: usize = 7;
        // const FASCET_COUNT: usize = 1;
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
            vertices.push(LogicalVertex {
                position: [
                    position.x.get_length() as f32,
                    position.y.get_length() as f32,
                    0.,
                ],
                color: [
                    self.background_color.r as f32,
                    self.background_color.g as f32,
                    self.background_color.b as f32,
                ],
            });
        }
        let number_of_triangles = vertices.len() - 2;
        let mut indices = Vec::with_capacity(number_of_triangles * 3);
        let mut widget_id = Vec::with_capacity(number_of_triangles);
        for i in 0..number_of_triangles {
            indices.push(indice_offset + 0);
            indices.push(indice_offset + (i + 1) as u16);
            indices.push(indice_offset + (i + 2) as u16);
            widget_id.push(self.id);
        }

        let polygon = Polygon {
            start_index: indice_offset as usize,
            end_index: indice_offset as usize + vertices.len(),
            widget_id: self.id,
            convex: true,
            rendered: true
        };

        (vertices, indices, polygon)
    }
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
