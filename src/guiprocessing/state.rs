use std::iter;

use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::window::Window;

use crate::guiproperties::guiposition::{GUIPosition, GUISize};
use crate::guiresources::GUIResources;
use crate::guiwidgets::GUIBase;

use crate::guiprocessing::processing_utils;
use crate::guiprocessing::vertices::LogicalVertex;

use super::vertices::{Polygon, Vertex};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    pub size: winit::dpi::PhysicalSize<u32>,

    pub guibase: GUIBase,

    logical_vertices: Vec<LogicalVertex>,
    polygons: Vec<Polygon>,
    pub curser_position: GUIPosition,
    clicked_widget_id: Option<u128>,
}

impl State {
    // pub async fn new(window: &Window, guiwindow: GUIWindow, guiresources: GUIResources) -> Self {
    pub async fn new(window: &Window, guibase: GUIBase, guiresources: GUIResources) -> Self {
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(guiresources.backend());
        // The surface is part of the window that's drawn to.
        let surface = unsafe { instance.create_surface(window) };
        // The adapter is the handle to the actual graphics card.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: guiresources.power_preference(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let (logical_vertices, polygons, vertex_buffer, index_buffer, num_indices) = make_wireframe_primitives(&guibase, &device);

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            size,
            guibase,
            logical_vertices,
            polygons,
            curser_position: GUIPosition::default(),
            clicked_widget_id: None,
        }
    }
    pub fn resize(&mut self, new_size: GUISize) {
        self.guibase.get_base_window_mut().size = new_size;
        self.config.width = new_size
            .width
            .get_physical_length(&self.guibase.logical_scale.unwrap())
            .round() as u32;
        self.config.height = new_size
            .height
            .get_physical_length(&self.guibase.logical_scale.unwrap())
            .round() as u32;
        self.surface.configure(&self.device, &self.config);
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    #[warn(dead_code)]
    pub fn update(&mut self) {}

    pub fn set_curser_position(&mut self, position: GUIPosition) {
        self.curser_position = position;
    }

    pub fn mouse_input(&mut self, button: &MouseButton) {
        use MouseButton::*;

        match button {
            Left => {
                let clicked_widget_id = processing_utils::get_clicked_widget(
                    &self.polygons,
                    &self.logical_vertices,
                    &self.curser_position,
                );
                match self.clicked_widget_id {
                    Some(last_id) => match clicked_widget_id {
                        Some(this_id) => {
                            if last_id == this_id {
                                println!("You clicked widget id {last_id}");
                            }
                            self.clicked_widget_id = None;
                        }
                        None => {
                            self.clicked_widget_id = None;
                        }
                    },
                    None => match clicked_widget_id {
                        Some(_) => {
                            self.clicked_widget_id = clicked_widget_id;
                        }
                        None => {
                            self.clicked_widget_id = None;
                        }
                    },
                }
            }
            Right => {
                println!("Right mouse button!");
            }
            Middle => {
                println!("Middle mouse button!");
            }
            Other(number) => {
                println!("Button: {number}");
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        if self.guibase.fixed_scale {
            let (logical_vertices, polygons, vertex_buffer, index_buffer, num_indices) = make_wireframe_primitives(&self.guibase, &self.device);
            self.logical_vertices = logical_vertices;
            self.polygons = polygons;
            self.vertex_buffer = vertex_buffer;
            self.index_buffer = index_buffer;
            self.num_indices = num_indices;
        }

        {
            let guiwindow = self.guibase.get_base_window();
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: guiwindow.background_color.r,
                            g: guiwindow.background_color.g,
                            b: guiwindow.background_color.b,
                            a: guiwindow.background_color.a,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            let data = (&self.vertex_buffer, &self.index_buffer, self.num_indices);
            render_pass.set_vertex_buffer(0, data.0.slice(..));
            render_pass.set_index_buffer(data.1.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..data.2, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn make_wireframe_primitives(guibase: &GUIBase, device: &Device) -> (Vec<LogicalVertex>, Vec<Polygon>, Buffer, Buffer, u32) {
    let (logical_vertices, indices, polygons) =
        processing_utils::make_vertices_and_indices(&guibase);
    let width = guibase.get_base_window().size.width.get_length() as f32;
    let height = guibase.get_base_window().size.height.get_length() as f32;
    let vertices: Vec<Vertex> = logical_vertices
        .iter()
        .map(|v| v.to_vertex(width, height))
        .collect();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        // contents: bytemuck::cast_slice(VERTICES),
        contents: bytemuck::cast_slice(&vertices[..]),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        // contents: bytemuck::cast_slice(INDICES),
        contents: bytemuck::cast_slice(&indices[..]),
        usage: wgpu::BufferUsages::INDEX,
    });
    // let num_indices = INDICES.len() as u32;
    let num_indices = indices.len() as u32;

    (logical_vertices, polygons, vertex_buffer, index_buffer, num_indices)
}
