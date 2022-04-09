use std::cmp;
use crate::guiproperties::guiposition::GUIPosition;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    // pub id: u128,
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

pub struct Triangles {
    pub triangles: Vec<[GUIPosition; 3]>,
    pub widget_id: Vec<u128>,
}

impl Triangles {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            widget_id: Vec::new(),
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.triangles.extend(other.triangles);
        self.widget_id.extend(other.widget_id);
    }

    pub fn get_triangles(&self) -> &Vec<[GUIPosition; 3]> {
        &self.triangles
    }

    pub fn get_widget_id(&self, position: &GUIPosition) -> Option<u128> {
        for i in 0..self.triangles.len() {
            // let one = self.triangles[3 * i + 0];
            // let two = self.triangles[3 * i + 1];
            // let three = self.triangles[3 * i + 2];
            let one = self.triangles[i][0];
            let two = self.triangles[i][1];
            let three = self.triangles[i][2];

            let minx = one.x.get_length().min(two.x.get_length()).min(three.x.get_length());
            let maxx = one.x.get_length().max(two.x.get_length()).max(three.x.get_length());
            let miny = one.y.get_length().min(two.y.get_length()).min(three.y.get_length());
            let maxy = one.y.get_length().max(two.y.get_length()).max(three.y.get_length());
            
            if minx < position.x.get_length() &&
                position.x.get_length() < maxx &&
                miny < position.y.get_length() && 
                position.y.get_length() < maxy {
                    if point_in_triangle((position.x.get_length() as f64, position.y.get_length() as f64),
                                         (one.x.get_length(), one.y.get_length()), 
                                         (two.x.get_length(), two.y.get_length()), 
                                         (three.x.get_length(), three.y.get_length())) {
                                             println!("clicked on: {:?}", self.widget_id[i]);
                                            return Some(self.widget_id[i])
                                         }
                } else {
                    continue;
                }
        }

        println!("didn't find a triangle!");

        None
    }
}

fn sign (p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> f64 {
    return (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1);
}

fn point_in_triangle (pt: (f64, f64), v1: (f64, f64), v2: (f64, f64), v3: (f64, f64)) -> bool {
    let d1 = sign(pt, v1, v2);
    let d2 = sign(pt, v2, v3);
    let d3 = sign(pt, v3, v1);

    let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
    let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

    !(has_neg && has_pos)
}
