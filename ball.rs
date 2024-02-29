use wgpu::{util::DeviceExt, Buffer, Device};

use crate::Vertex;

const X: usize = 0;
const Y: usize = 0;

pub struct Ball {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    x_vel: f32,
    y_vel: f32,
}

impl Ball {
    pub fn new(device: &Device) -> Self {
        let vertices = vec![
            Vertex {
                position: [-0.01, 0.02, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 0
            Vertex {
                position: [0.02, 0.02, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 1
            Vertex {
                position: [-0.01, -0.02, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 2
            Vertex {
                position: [0.02, -0.02, 0.0],
                color: [0.0, 0.0, 0.0, 0.0],
            }, // 3
        ];
        let indices = vec![2, 1, 0, 1, 2, 3];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ball Vertex Buffer"),
            contents: bytemuck::cast_slice(&*vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ball Index Buffer"),
            contents: bytemuck::cast_slice(&*indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            x_vel: -0.1, // Start the ball moving left
            y_vel: 0.0,
        }
    }

    pub fn move_ball(&mut self, x_offset: f32, y_offset: f32) {
        for vertex in &mut self.vertices {
            vertex.position[X] += x_offset;
            vertex.position[Y] += y_offset;
        }
    }
}
