use bevy::prelude::*;
/// Convert 1 based file and rank indicies into 3D coordinates
pub fn file_rank(file: u8, rank: u8) -> Vec3 {
    Vec3::new((rank - 1) as f32, 0., (file - 1) as f32)
}
