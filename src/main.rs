use bevy::prelude::*;
use global_assets::{GlobalAssets, GlobalAssetsPlugin};

mod global_assets;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4})
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
        .run();
}

fn setup (
    mut cmd: Commands,
) {
    
    let cam_pos   = Vec3::new(-7., 20., 4.);
    let light_pos = Vec3::new(4., 8., 4.);

    let cam_rot = Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize();
    
    // Camera
    cmd
        .spawn_bundle(PerspectiveCameraBundle {
            transform: 
            Transform {
                translation: cam_pos,
                rotation: cam_rot,
                ..Default::default()
            },
            ..Default::default()
        });
    
    // Light
    cmd
        .spawn_bundle(LightBundle {
            transform: Transform {
                translation: light_pos,
                ..Default::default()
            },
            ..Default::default()
        });
}

fn create_board (
    mut cmd: Commands,
        global_assets: Res<GlobalAssets>,
) {

    for i in 0..8 {
        for j in 0..8 {
            let transform = Transform::from_translation(Vec3::new(i as f32, 0., j as f32));
            let mesh = global_assets.mesh_tile.clone();
            let material: Handle<StandardMaterial>;

            if (i + j + 1) % 2 == 0 {
                material = global_assets.mat_white_tile.clone();
            } else {
                material = global_assets.mat_black_tile.clone();
            }

            cmd
                .spawn_bundle(PbrBundle { transform, mesh, material, ..Default::default() });

        }
    }

}