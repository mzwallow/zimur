use crate::math::Real;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// Holds position of the vertex in counter-clockwise
    /// order: top, bottom left, bottom right.
    pub position: [Real; 3],
    pub tex_coords: [Real; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // Width of a Vertex, about 24 bytes.
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            // 1:1 mapping with a struct's field.
            //
            // We can also use wgpu::vertex_attr_array!.
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // Tell shader what location ot store this attribute at.
                    shader_location: 0, // @location(0) x: vec3<f32> => position
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[Real; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
