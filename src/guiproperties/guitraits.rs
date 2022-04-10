use std::any::Any;

use crate::guiprocessing::vertices::{LogicalVertex, Polygon};
use crate::guiproperties::guiposition::{GUIPosition, GUISize};

pub trait Widget {
    fn get_vertices_and_indices(
        &self,
        parent_position: &GUIPosition,
        indice_offset: u16,
    ) -> (Vec<LogicalVertex>, Vec<u16>, Polygon);

    fn get_size(&self) -> &GUISize;

    fn get_position(&self) -> &GUIPosition;

    fn get_id(&self) -> &u128;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Parent: Widget {}
