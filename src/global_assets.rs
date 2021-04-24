use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlobalMesh {
    Tile,
    King,
    KingCross,
    Pawn,
    Knight1,
    Knight2,
    Rook,
    Bishop,
    Queen,
}
use GlobalMesh::*;

#[derive(Debug, PartialEq)]
pub enum GlobalMaterial {
    BlackTile,
    WhiteTile,
    BlackPiece,
    WhitePiece,
}
use GlobalMaterial::*;

pub struct GlobalAssets {
    pub mesh_tile: Handle<Mesh>,

    pub mesh_king:       Handle<Mesh>,
    pub mesh_king_cross: Handle<Mesh>,
    pub mesh_pawn:       Handle<Mesh>,
    pub mesh_knight_1:   Handle<Mesh>,
    pub mesh_knight_2:   Handle<Mesh>,
    pub mesh_rook:       Handle<Mesh>,
    pub mesh_bishop:     Handle<Mesh>,
    pub mesh_queen:      Handle<Mesh>,

    pub mat_black_tile: Handle<StandardMaterial>,
    pub mat_white_tile: Handle<StandardMaterial>,

    pub mat_black_piece: Handle<StandardMaterial>,
    pub mat_white_piece: Handle<StandardMaterial>,
}

impl GlobalAssets {
    pub fn clone_mesh(&self, mesh: GlobalMesh) -> Handle<Mesh> {
        match mesh {
            Tile => self.mesh_tile.clone(),
            King => self.mesh_king.clone(),
            KingCross => self.mesh_king_cross.clone(),
            Pawn => self.mesh_pawn.clone(),
            Knight1 => self.mesh_knight_1.clone(),
            Knight2 => self.mesh_knight_2.clone(),
            Rook => self.mesh_rook.clone(),
            Bishop => self.mesh_bishop.clone(),
            Queen => self.mesh_queen.clone(),
        }
    }

    pub fn clone_material(&self, material: GlobalMaterial) -> Handle<StandardMaterial> {
        match material {
            BlackTile => self.mat_black_tile.clone(),
            WhiteTile => self.mat_white_tile.clone(),
            BlackPiece => self.mat_black_piece.clone(),
            WhitePiece => self.mat_white_piece.clone(),
        }
    }
}

pub struct GlobalAssetsPlugin;

impl GlobalAssetsPlugin {
    fn setup(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        cmd.insert_resource(GlobalAssets {
            mesh_tile: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),

            mesh_king:       asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0"),
            mesh_king_cross: asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0"),
            mesh_pawn:       asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0"),
            mesh_knight_1:   asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0"),
            mesh_knight_2:   asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0"),
            mesh_rook:       asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0"),
            mesh_bishop:     asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0"),
            mesh_queen:      asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0"),

            mat_black_tile: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            mat_white_tile: materials.add(Color::rgb(0., 0.1, 0.1).into()),

            mat_black_piece: materials.add(Color::rgb(0., 0.2, 0.2).into()),
            mat_white_piece: materials.add(Color::rgb(1., 0.8, 0.8).into()),
        })
    }
}

impl Plugin for GlobalAssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::setup.system());
    }
}
