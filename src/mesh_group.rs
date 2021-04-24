use crate::prelude::*;

///
/// A collection of global meshes combined with transformation and scaling information.
///
pub struct MeshGroup(&'static [Entry]);

const MESH_GROUP_KING:   MeshGroup = MeshGroup(&[Entry(GlobalMesh::King, -0.2, 0., -1.95, 0.2, 0.2, 0.2),
                                                 Entry(GlobalMesh::KingCross, -0.2, 0., -1.95, 0.2, 0.2, 0.2)]);
const MESH_GROUP_QUEEN:  MeshGroup = MeshGroup(&[Entry(GlobalMesh::Queen, -0.2, 0., -0.95, 0.2, 0.2, 0.2)]);
const MESH_GROUP_BISHOP: MeshGroup = MeshGroup(&[Entry(GlobalMesh::Bishop, -0.1, 0., 0., 0.2, 0.2, 0.2)]);
const MESH_GROUP_KNIGHT: MeshGroup = MeshGroup(&[Entry(GlobalMesh::Knight1, -0.2, 0., 0.9, 0.2, 0.2, 0.2),
                                                 Entry(GlobalMesh::Knight2, -0.2, 0., 0.9, 0.2, 0.2, 0.2)]);
const MESH_GROUP_ROOK:   MeshGroup = MeshGroup(&[Entry(GlobalMesh::Rook, -0.1, 0., 1.8, 0.2, 0.2, 0.2)]);
const MESH_GROUP_PAWN:   MeshGroup = MeshGroup(&[Entry(GlobalMesh::Pawn, -0.2, 0., 2.6, 0.2, 0.2, 0.2)]);

impl From<Piece> for MeshGroup {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::King   => MESH_GROUP_KING,
            Piece::Queen  => MESH_GROUP_QUEEN,
            Piece::Bishop => MESH_GROUP_BISHOP,
            Piece::Knight => MESH_GROUP_KNIGHT,
            Piece::Rook   => MESH_GROUP_ROOK,
            Piece::Pawn   => MESH_GROUP_PAWN,
        }
    }
}

pub trait MeshGroupCommands {
    fn spawn_mesh_group_meshes(&mut self,
                               mesh_group: MeshGroup,
                               mat: Handle<StandardMaterial>,
                               global_assets: &GlobalAssets);
}

impl MeshGroupCommands for ChildBuilder<'_, '_> {
    fn spawn_mesh_group_meshes(
        &mut self,
             mesh_group: MeshGroup,
             mat: Handle<StandardMaterial>,
             global_assets: &GlobalAssets
    ) {
        for entry in mesh_group.0 {
            let &Entry(global_mesh, x, y, z, sx, sy, sz) = entry;
            let mesh = global_assets.clone_mesh(global_mesh);
            let material = mat.clone();
            self.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(x, y, z));
                    transform.apply_non_uniform_scale(Vec3::new(sx, sy, sz));
                    transform
                },
                ..Default::default()
            });
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Entry(GlobalMesh, f32, f32, f32, f32, f32, f32);