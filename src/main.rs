use global_assets::{
    GlobalAssets,
    GlobalAssetsPlugin,
};
use pieces::create_pieces;
use prelude::*;

pub mod board_layout;
pub mod chess;
pub mod global_assets;
pub mod mesh_group;
pub mod pieces;
pub mod prelude;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GlobalAssetsPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
        .run();
}

fn setup(mut cmd: Commands) {
    let cam_pos = Vec3::new(-7., 20., 4.);
    let light_pos = Vec3::new(4., 8., 4.);

    let cam_rot = Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize();

    // Camera
    cmd.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform {
            translation: cam_pos,
            rotation: cam_rot,
            ..Default::default()
        },
        ..Default::default()
    });

    // Light
    cmd.spawn_bundle(LightBundle {
        transform: Transform {
            translation: light_pos,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn create_board(mut cmd: Commands, global_assets: Res<GlobalAssets>) {
    use board_layout::*;

    for i in 1..=8 {
        for j in 1..=8 {
            let transform = Transform::from_translation(file_rank(i, j));
            let mesh = global_assets.mesh_tile.clone();
            let material: Handle<StandardMaterial>;

            if (i + j) % 2 == 0 {
                material = global_assets.mat_white_tile.clone();
            } else {
                material = global_assets.mat_black_tile.clone();
            }

            cmd.spawn_bundle(PbrBundle {
                transform,
                mesh,
                material,
                ..Default::default()
            });
        }
    }
}
