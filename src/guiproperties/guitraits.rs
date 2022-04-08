use crate::guiprocessing::vertices::Vertex;
use crate::guiproperties::guiposition::GUISize;

pub trait Widget {
    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        indice_offset: u16,
    ) -> (Vec<Vertex>, Vec<u16>);

    fn get_size(&self) -> &GUISize;
}
