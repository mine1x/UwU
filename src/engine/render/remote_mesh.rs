use crate::network::RemotePlayer;
use crate::render::types::Vertex;

pub fn append_remote_player_meshes(
    v: &mut Vec<Vertex>,
    i: &mut Vec<u32>,
    remote_players: &[RemotePlayer],
) {
    for rp in remote_players {
        let (rv, ri) = rp.to_player().mesh();
        let offset = v.len() as u32;
        v.extend_from_slice(&rv);
        for idx in ri {
            i.push(offset + idx);
        }
    }
}
