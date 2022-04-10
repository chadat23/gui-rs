#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct LogicalVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl LogicalVertex {
    pub fn to_vertex(&self, width: f32, height: f32) -> Vertex {
        let mut position = [0.0_f32; 3];
        position[0] = self.position[0] / width * 2. - 1.;
        position[1] = -self.position[1] / height * 2. + 1.;
        let mut color = [0.0_f32; 3];
        color[0] = self.color[0];
        color[1] = self.color[1];
        color[2] = self.color[2];

        Vertex { position, color }
    }
}

pub struct Polygon {
    pub start_index: usize,
    pub end_index: usize,
    pub widget_id: u128,
    pub convex: bool,
    pub rendered: bool,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            start_index: 0,
            end_index: 0,
            widget_id: 0,
            convex: true,
            rendered: false,
        }
    }
}
