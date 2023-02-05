use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::prelude::*;

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
            .add_plugin(Material2dPlugin::<GridMaterial>::default())
            .add_startup_system(spawn_background)
            .add_system_set(SystemSet::on_update(PlayerState::Building).with_system(spawn_grid));
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0),
            ..Default::default()
        },
        material: materials.add(BackgroundMaterial {}),
        ..Default::default()
    });
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0),
                ..Default::default()
            },
            material: materials.add(GridMaterial {}),
            ..Default::default()
        },
        ForState {
            states: vec![AppState::Game],
        },
        ForState {
            states: vec![PlayerState::Building],
        },
    ));
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {}

impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        "background.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "c5ff85a8-583f-41a4-b4a4-0c579e8a8811"]
struct GridMaterial {}

impl Material2d for GridMaterial {
    fn vertex_shader() -> ShaderRef {
        "grid.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "grid.wgsl".into()
    }
}
