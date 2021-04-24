use crate::{mesh_group::MeshGroupCommands, prelude::*};

pub fn create_pieces(
    mut cmd: Commands,
        global_assets: Res<GlobalAssets>,
) {
    use board_layout::*;
    let ga = &global_assets;

    //
    // White
    //
    cmd.spawn_piece(ga, Piece::Rook,   PieceColor::White, file_rank(1, 1));
    cmd.spawn_piece(ga, Piece::Knight, PieceColor::White, file_rank(2, 1));
    cmd.spawn_piece(ga, Piece::Bishop, PieceColor::White, file_rank(3, 1));
    cmd.spawn_piece(ga, Piece::Queen,  PieceColor::White, file_rank(4, 1));
    cmd.spawn_piece(ga, Piece::King,   PieceColor::White, file_rank(5, 1));
    cmd.spawn_piece(ga, Piece::Bishop, PieceColor::White, file_rank(6, 1));
    cmd.spawn_piece(ga, Piece::Knight, PieceColor::White, file_rank(7, 1));
    cmd.spawn_piece(ga, Piece::Rook,   PieceColor::White, file_rank(8, 1));
    for i in 1..=8 {
        cmd.spawn_piece(ga, Piece::Pawn, PieceColor::White, file_rank(i, 2))
    }

    //
    // Black
    //
    cmd.spawn_piece(ga, Piece::Rook,   PieceColor::Black, file_rank(1, 8));
    cmd.spawn_piece(ga, Piece::Knight, PieceColor::Black, file_rank(2, 8));
    cmd.spawn_piece(ga, Piece::Bishop, PieceColor::Black, file_rank(3, 8));
    cmd.spawn_piece(ga, Piece::Queen,  PieceColor::Black, file_rank(4, 8));
    cmd.spawn_piece(ga, Piece::King,   PieceColor::Black, file_rank(5, 8));
    cmd.spawn_piece(ga, Piece::Bishop, PieceColor::Black, file_rank(6, 8));
    cmd.spawn_piece(ga, Piece::Knight, PieceColor::Black, file_rank(7, 8));
    cmd.spawn_piece(ga, Piece::Rook,   PieceColor::Black, file_rank(8, 8));
    for i in 1..=8 {
        cmd.spawn_piece(ga, Piece::Pawn, PieceColor::Black, file_rank(i, 7))
    }
}

/// Helper to convert file and rank (1-8) into a 3D location

trait SpawnPieceCommands {
    fn spawn_piece(
        &mut self,
             global_assets: &GlobalAssets,
             piece: Piece,
             color: PieceColor,
             loc: Vec3
    );
}

impl SpawnPieceCommands for Commands<'_> {
    fn spawn_piece(
        &mut self,
             global_assets: &GlobalAssets,
             piece: Piece,
             color: PieceColor,
             loc: Vec3
    ) {

        let mesh_group = piece.into();
        let mat = match color {
            PieceColor::Black => global_assets.clone_material(GlobalMaterial::BlackPiece),
            PieceColor::White => global_assets.clone_material(GlobalMaterial::WhitePiece),
        };
        
        self
            .spawn_bundle(PbrBundle {
                transform: Transform::from_translation(loc),
                ..Default::default()
            })
            .with_children(|p| p.spawn_mesh_group_meshes(mesh_group, mat, global_assets));
    }
}