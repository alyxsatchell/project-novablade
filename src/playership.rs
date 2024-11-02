use crystal_nova::{object::{Object, Placement}, universe::Vertex};
use wgpu::{util::DeviceExt, BufferSlice, Device};

pub struct PlayerShip<'a> {
    placement: Placement,
    vertices: &'a[Vertex],
    vertex_buffer: Option<wgpu::Buffer>,
    indices: &'a[u16],
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
}

impl<'a> PlayerShip<'a>{
    pub fn new() -> PlayerShip<'a>{
        let placement = Placement{ x: 0., y: 0., z: 0. };
        let vertices = &[
            Vertex {
                position: [0., 0.25, 0.],
            },
            Vertex {
                position: [-0.1, -0.25, 0.],
            },
            Vertex {
                position: [0., -0.1, 0.],
            },
            Vertex {
                position: [0.1, -0.25, 0.],
            },
        ];
        let indices = &[0,1,2,0,1,3];
        let num_indices = indices.len() as u32;
        PlayerShip { placement, vertices, vertex_buffer: None, indices, index_buffer: None, num_indices}
    }

    pub fn init(&mut self, device: wgpu::Device){
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));
        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(self.indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
    }
}

impl<'a> Object for PlayerShip<'a>{
    fn Up(&mut self) {
        self.placement.y += 0.1;
    }

    fn Down(&mut self) {
        self.placement.y -= 0.1;
    }

    fn Left(&mut self) {
        self.placement.x -= 0.1;
    }

    fn Right(&mut self) {
        self.placement.x += 0.1;
    }

    fn placement(&self) -> &Placement {
        &self.placement
    }

    fn num_indices(&self) -> u32 {
        self.num_indices
    }
    
    fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }
    
    fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }
}