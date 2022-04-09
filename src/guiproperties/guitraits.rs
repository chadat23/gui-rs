use std::any::Any;

use crate::guiprocessing::vertices::{Triangles, Vertex};
use crate::guiproperties::guiposition::GUISize;

pub trait Widget {
    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        indice_offset: u16,
    ) -> (Vec<Vertex>, Vec<u16>, Triangles);

    fn get_size(&self) -> &GUISize;

    fn get_id(&self) -> &u128;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Parent: Widget {}
