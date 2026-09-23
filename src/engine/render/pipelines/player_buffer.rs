use wgpu::util::DeviceExt;
use crate::network::RemotePlayer;
use crate::render::types::Vertex;

pub fn update_player_buffers(
    device: &wgpu::Device,
    local_mesh: &(Vec<Vertex>, Vec<u32>),
    remote_players: &[RemotePlayer],
) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let mut v = local_mesh.0.clone();
    let mut i = local_mesh.1.clone();
    crate::render::remote_mesh::append_remote_player_meshes(&mut v, &mut i, remote_players);

    let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("PV"), contents: bytemuck::cast_slice(&v), usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });
    let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("PI"), contents: bytemuck::cast_slice(&i), usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    });
    let count = i.len() as u32;
    (vb, ib, count)
}
