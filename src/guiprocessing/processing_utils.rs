use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::vertices::Vertex;
// use crate::guiproperties::guiposition::GUISize;
// use crate::guiproperties::guitraits::*;
use crate::{
    guiproperties::{guiposition::GUISize, guitraits::Widget},
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

// pub fn _make_vertices_and_indices(
//     parent_size: &GUISize,
//     children: &Vec<Box<dyn Family>>,
// ) -> (Vec<Vertex>, Vec<u16>) {
//     let mut all_vertices: Vec<Vertex> = Vec::new();
//     let mut all_indices: Vec<u16> = Vec::new();
//     for child in children {
//         let (vertices, indices) = child.get_vertices_and_indices(&parent_size, all_vertices.len() as u16);
//         all_vertices.extend(vertices);
//         all_indices.extend(indices);
//         for child in child.get_children() {
//             let (vertices, indices) = child.get_vertices_and_indices(&parent_size, all_vertices.len() as u16);
//             all_vertices.extend(vertices);
//             all_indices.extend(indices);
//         }
//     };
//     (all_vertices, all_indices)
// }

pub fn make_vertices_and_indices(guibase: &GUIBase) -> (Vec<Vertex>, Vec<u16>) {
    let mut all_vertices: Vec<Vertex> = Vec::new();
    let mut all_indices: Vec<u16> = Vec::new();
    let mut index_offset = 0;

    for (_, gwindow) in guibase.windows.iter() {
        let child_ids = gwindow.get_child_ids();
        // let size = gwindow.get_window().get_size();
        for child_id in child_ids {
            let (vertices, indices) = make_child(
                &guibase,
                gwindow.get_window().get_size(),
                child_id,
                index_offset,
            );
            index_offset = vertices.len() as u16;
        }
    }

    (all_vertices, all_indices)
}

fn make_child(guibase: &GUIBase, parent_size: &GUISize, widget_id: &i128, index_offset: u16) -> (Vec<Vertex>, Vec<u16>) {
    let mut all_vertices: Vec<Vertex> = Vec::new();
    let mut all_indices: Vec<u16> = Vec::new();
    let gwidget = guibase.widgets.get(widget_id).unwrap();
    let (vertices, indices) = gwidget
        .get_widget()
        .get_vertices_and_indices(parent_size, index_offset);
    all_vertices.extend(vertices);
    all_indices.extend(indices);
    for child_id in gwidget.get_child_ids() {
        let (vertices, indices) = make_child(
            &guibase,
            gwidget.get_widget().get_size(),
            child_id,
            all_vertices.len() as u16,
        );
        all_vertices.extend(vertices);
        all_indices.extend(indices);
    }

    (all_vertices, all_indices)
}
