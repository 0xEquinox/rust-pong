use wgpu::{util::DeviceExt, Device};

use crate::Vertex;

const X: usize = 0;
const Y: usize = 1;


pub struct Player {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u16>,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
}

impl Player {
    pub fn new(device: &Device) -> Self {
        let vertices = vec![
            Vertex {
                position: [-0.925, 0.2, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 0
            Vertex {
                position: [-0.9, 0.2, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 1
            Vertex {
                position: [-0.925, -0.2, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 2
            Vertex {
                position: [-0.9, -0.2, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 3
        ];
        let indices: Vec<u16> = vec![2, 1, 0, 1, 2, 3];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Player Vertex Buffer"),
            contents: bytemuck::cast_slice(&*vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Player Index Buffer"),
            contents: bytemuck::cast_slice(&*indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn move_vertically(&mut self, vert_offs: f32) {
        if self.vertices[0].position[Y] >= 1.0 || self.vertices[2].position[Y] <= -1.0 {
            return;
        }
        for vertex in &mut self.vertices {
            vertex.position[Y] += vert_offs;
        }
    }

}
