use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::vertices::{LogicalVertex, Polygon};
use crate::{
    guiproperties::{
        guiposition::{GUIPosition, GUISize},
        guitraits::Widget,
    },
    guiwidgets::{GUIBase, GUIWindow},
};

pub fn set_window_properties(window: Window, guibase: &GUIBase, guiwindow: &GUIWindow) -> Window {
    window.set_title(guiwindow.title);
    // window.set_inner_size(PhysicalSize::new(width: 8, height: 8));
    window.set_inner_size(PhysicalSize::new(
        guiwindow
            .size
            .width
            .get_physical_length(&guibase.logical_scale.unwrap()) as u32,
        guiwindow
            .size
            .height
            .get_physical_length(&guibase.logical_scale.unwrap()) as u32,
    ));
    window.set_min_inner_size(Some(PhysicalSize::new(
        guiwindow
            .min_size
            .width
            .get_physical_length(&guibase.logical_scale.unwrap()) as u32,
        guiwindow
            .min_size
            .height
            .get_physical_length(&guibase.logical_scale.unwrap()) as u32,
    )));
    // window.set_max_inner_size(Some(PhysicalSize::new(
    //     guiwindow
    //         .max_size
    //         .width
    //         .get_physical_length(&guibase.logical_scale.unwrap()) as u32,
    //     guiwindow
    //         .max_size
    //         .height
    //         .get_physical_length(&guibase.logical_scale.unwrap()) as u32,
    // )));
    window
}

pub fn make_vertices_and_indices(
    guibase: &GUIBase,
) -> (Vec<LogicalVertex>, Vec<u16>, Vec<Polygon>) {
    let mut all_vertices: Vec<LogicalVertex> = Vec::new();
    let mut all_indices: Vec<u16> = Vec::new();
    // let mut all_triangles: Triangles = Triangles::new();
    let mut all_polygons: Vec<Polygon> = Vec::new();
    let mut index_offset = 0;

    for (_, gwindow) in guibase.windows.iter() {
        let child_ids = gwindow.get_child_ids();
        // let size = gwindow.get_window().get_size();
        for child_id in child_ids {
            let (vertices, indices, polygons) = make_child(
                &guibase,
                gwindow.get_window().get_position(),
                child_id,
                index_offset,
            );
            all_vertices.extend(vertices);
            all_indices.extend(indices);
            all_polygons.extend(polygons);
            // all_triangles.extend(triangles);
            index_offset = all_vertices.len() as u16;
        }
    }

    (all_vertices, all_indices, all_polygons)
}

fn make_child(
    guibase: &GUIBase,
    parent_position: &GUIPosition,
    widget_id: &u128,
    index_offset: u16,
) -> (Vec<LogicalVertex>, Vec<u16>, Vec<Polygon>) {
    let mut all_vertices: Vec<LogicalVertex> = Vec::new();
    let mut all_indices: Vec<u16> = Vec::new();
    // let mut all_triangles: Triangles = Triangles::new();
    let mut all_polygons: Vec<Polygon> = Vec::new();
    let gwidget = guibase.widgets.get(widget_id).unwrap();
    let (vertices, indices, polygon) = gwidget
        .get_widget()
        .get_vertices_and_indices(parent_position, index_offset);
    all_vertices.extend(vertices);
    // all_indices.extend(indices.iter());
    all_indices.extend(indices);
    // all_triangles.extend(triangles);
    all_polygons.push(polygon);

    for child_id in gwidget.get_child_ids() {
        let (vertices, indices, polygons) = make_child(
            &guibase,
            gwidget.get_widget().get_position(),
            child_id,
            all_vertices.len() as u16,
        );
        all_vertices.extend(vertices);
        all_indices.extend(indices);
        // all_triangles.extend(triangles);
        all_polygons.extend(polygons);
    }

    (all_vertices, all_indices, all_polygons)
}

pub fn get_clicked_widget(
    polygons: &Vec<Polygon>,
    vertices: &Vec<LogicalVertex>,
    position: &GUIPosition,
) -> Option<u128> {
    let px = position.x.get_length() as f32;
    let py = position.y.get_length() as f32;

    for polygon in polygons.iter().rev() {
        if polygon.rendered {
            if is_inside_polygon(&vertices[polygon.start_index..polygon.end_index], px, py) {
                return Some(polygon.widget_id);
            }
        }
    }
    None
}

fn is_inside_polygon(vertices: &[LogicalVertex], px: f32, py: f32) -> bool {
    // https://youtu.be/01E0RGb2Wzo
    let mut cross_count = 0u32;
    let [mut last_x, mut last_y, _] = vertices.last().unwrap().position;
    for vertex in vertices {
        let [this_x, this_y, _] = vertex.position;
        cross_count += does_cross(last_x, last_y, this_x, this_y, px, py);
        last_x = this_x;
        last_y = this_y;
    }

    (cross_count & 1) == 1
}

fn does_cross(p1x: f32, p1y: f32, p2x: f32, p2y: f32, px: f32, py: f32) -> u32 {
    const SAFETY_FACTOR: f32 = 1.0000001;

    let min_y = p1y.min(p2y);
    let max_y = p1y.max(p2y);

    if min_y / SAFETY_FACTOR < py && py < max_y * SAFETY_FACTOR {
        let slope = (p1y - p2y) / (p1x - p2x);
        let x_projection_interce = (py - p1y) / slope + p1x;

        if px < x_projection_interce * SAFETY_FACTOR {
            return 1;
        }
    }
    0
}
