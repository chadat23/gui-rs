use super::widget_utils;
use super::widget_utils::arcs;
use crate::guiprocessing::vertices::Vertex;
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
    pub name: &'static str,
    pub id: i128,
}

impl Widget for GUIButton {
    fn get_size(&self) -> &GUISize {
        &self.size
    }

    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        indice_offset: u16,
    ) -> (Vec<Vertex>, Vec<u16>) {
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
                id: self.id as i128,
            });
        }
        let number_of_triangles = vertices.len() - 2;
        let mut indices = Vec::with_capacity(number_of_triangles * 3);
        for i in 0..number_of_triangles {
            indices.push(indice_offset + 0);
            indices.push(indice_offset + (i + 1) as u16);
            indices.push(indice_offset + (i + 2) as u16);
        }

        (vertices, indices)
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
