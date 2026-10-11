//! The fox example with convex hull outline.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::collapsible_if)]
use std::f32::consts::PI;
use std::sync::OnceLock;

use bevy::gltf::GltfPlugin;
use bevy::mesh::skinning::SkinnedMesh;
use bevy::mesh::{MeshVertexAttribute, VertexFormat};
use bevy::pbr::ExtendedMaterial;
use bevy::world_serialization::WorldInstanceReady;
use bevy::{
    asset::{Asset, Handle},
    pbr::MaterialExtension,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, Face},
    shader::{Shader, ShaderRef},
};
use bevy::{light::CascadeShadowConfigBuilder, prelude::*};

pub static CONVEX_HULL_NORMAL: MeshVertexAttribute =
    MeshVertexAttribute::new("Convex_Hull_Normal", 666, VertexFormat::Float32x3);

pub static CONVEX_HULL_VERTEX_SHADER: OnceLock<Handle<Shader>> = OnceLock::new();

#[derive(Debug, Clone, Copy, AsBindGroup, Asset, Default, TypePath)]
pub struct ConvexHull {
    #[uniform(100)]
    pub thickness: f32,
}

impl MaterialExtension for ConvexHull {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Handle(CONVEX_HULL_VERTEX_SHADER.get().cloned().unwrap())
    }
    fn specialize(
        _: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = Some(Face::Front);
        let vertex_layout = layout
            .0
            .get_layout(&[CONVEX_HULL_NORMAL.at_shader_location(20)])?;
        //dbg!(&descriptor.vertex.buffers);
        descriptor.vertex.buffers[0]
            .attributes
            .push(vertex_layout.attributes[0]);
        //dbg!(&descriptor.vertex.buffers);
        Ok(())
    }
}

// An example asset that contains a mesh and animation.
const GLTF_PATH: &str = "Fox.glb";

fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
            ..default()
        })
        .add_plugins(
            DefaultPlugins.set(
                GltfPlugin::default()
                    .add_custom_vertex_attribute("CONVEX_HULL_NORMAL", CONVEX_HULL_NORMAL),
            ),
        )
        .add_plugins(|app: &mut App| {
            let _ = CONVEX_HULL_VERTEX_SHADER.set(
                app.world_mut()
                    .add_asset(Shader::from_wesl(include_str!("convex_hull.wesl"), "convex_hull.wesl")),
            );
        })
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, ConvexHull>,
        >::default())
        .add_systems(Startup, setup_mesh_and_animation)
        .add_systems(Startup, setup_camera_and_environment)
        .run();
}

// A component that stores a reference to an animation we want to play. This is
// created when we start loading the mesh (see `setup_mesh_and_animation`) and
// read when the mesh has spawned (see `play_animation_once_loaded`).
#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

fn setup_mesh_and_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(GLTF_PATH)),
    );

    let graph_handle = graphs.add(graph);

    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    let mesh_scene =
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));

    commands
        .spawn((animation_to_play, mesh_scene))
        .observe(when_ready);
}

fn when_ready(
    scene_ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
    meshes: ResMut<Assets<Mesh>>,
    mut shaders: ResMut<Assets<ExtendedMaterial<StandardMaterial, ConvexHull>>>,
    items: Query<
        (Entity, &Mesh3d, Option<&ChildOf>, Option<&SkinnedMesh>),
        With<MeshMaterial3d<StandardMaterial>>,
    >,
) {
    let shader = shaders.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            ..Default::default()
        },
        extension: ConvexHull { thickness: 1.0 },
    });
    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
        for child in children.iter_descendants(scene_ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                player.play(animation_to_play.index).repeat();
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
            if let Ok((_entity, mesh, child_of, skinned)) = items.get(child) {
                if meshes
                    .get(mesh)
                    .is_some_and(|x| x.contains_attribute(CONVEX_HULL_NORMAL))
                {
                    //commands.entity(entity).despawn();
                    let mut entity = commands.spawn((mesh.clone(), MeshMaterial3d(shader.clone())));
                    if let Some(child_of) = child_of {
                        entity.insert(child_of.clone());
                    }
                    if let Some(skinned) = skinned {
                        entity.insert(skinned.clone());
                    }
                }
            }
        }
    }
}

fn setup_camera_and_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(100.0, 100.0, 150.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    commands.spawn((
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .build(),
    ));
}
