use std::f32::consts::PI;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::{close_on_esc, PresentMode},
};

const BACKGROUND_COLOR: Color = Color::WHITE;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    title: "App".into(),
                    resizable: true,
                    present_mode: PresentMode::AutoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }).set(
                AssetPlugin {
                    watch_for_changes_override: Some(true),
            
                    ..Default::default()
                }
            ),
            Material2dPlugin::<Custom2dMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        // .add_systems(Fixed, )
        .add_systems(Update, (close_on_esc, rotate_cube))
        .run();
}

#[derive(Component)]
pub struct Cube;

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut materials2: ResMut<Assets<Custom2dMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(
                shape::Quad::new(Vec2 {
                    x: SCREEN_WIDTH,
                    y: SCREEN_HEIGHT,
                })
                .into(),
            )
            .into(),
        material: materials2.add(Custom2dMaterial {}),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });
}

#[allow(dead_code)]
fn setup1(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut materials2: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let cube_handle = meshes.add(Mesh::from(shape::Cube::new(2.0)));

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        // alpha_mode: AlphaMode::Opaque,
        metallic: 1.0,
        perceptual_roughness: 0.1,
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: cube_handle.clone(),
            material: material_handle,
            transform: Transform::from_xyz(0.1, 0.0, 0.0),
            // .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..Default::default()
        },
        Cube,
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(8.0, 10.0, 8.1),
        point_light: PointLight {
            intensity: 6000000.,
            range: 100.,
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(8.0, -10.0, -8.1),
        point_light: PointLight {
            intensity: 6000000.,
            range: 100.,
            ..default()
        },
        ..default()
    });

    // commands.add(PbrBundle {  })
}

fn rotate_cube(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(-time.delta_seconds());
        transform.rotate_x(time.delta_seconds());
    }
}

// #[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "9c5a0ddf-1eaf-41b4-9832-ed736fd26af3"]
struct Custom2dMaterial {
    // alpha_mode: AlphaMode,
}
//
impl Material2d for Custom2dMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/hello.wgsl".into()
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/hello.wgsl".into()
    }

    // fn specialize(
    //     descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
    //     layout: &bevy::render::mesh::MeshVertexBufferLayout,
    //     key: bevy::sprite::Material2dKey<Self>,
    // ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
    //     Ok(())
    // }
}
