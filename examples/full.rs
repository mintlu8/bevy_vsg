use bevy::{
    mesh::{MeshVertexAttribute, VertexFormat},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

pub static ATTR_MAGIC_NUMBER1: MeshVertexAttribute =
    MeshVertexAttribute::new("Magic_Number_1", 501, VertexFormat::Float32);
pub static ATTR_MAGIC_NUMBER2: MeshVertexAttribute =
    MeshVertexAttribute::new("Magic_Number_2", 502, VertexFormat::Float32);
pub static ATTR_MAGIC_NUMBER3: MeshVertexAttribute =
    MeshVertexAttribute::new("Magic_Number_3", 503, VertexFormat::Float32);
pub static ATTR_MAGIC_NUMBER4: MeshVertexAttribute =
    MeshVertexAttribute::new("Magic_Number_4", 504, VertexFormat::Float32);


pub fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, RenderMagicNumber>,
        >::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Clone, AsBindGroup, Asset, Default, TypePath)]
pub struct RenderMagicNumber {}

impl MaterialExtension for RenderMagicNumber {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Path("full.wesl".into())
    }

    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("full.wesl".into())
    }

    fn specialize(
        _: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> std::result::Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[
                ATTR_MAGIC_NUMBER1.at_shader_location(20),
                ATTR_MAGIC_NUMBER2.at_shader_location(21),
                ATTR_MAGIC_NUMBER3.at_shader_location(22),
                ATTR_MAGIC_NUMBER4.at_shader_location(23),
            ])?;
        descriptor.vertex.buffers[0]
            .attributes
            .extend(vertex_layout.attributes);
        Ok(())
    }
}

fn setup(
    mut commands: Commands<'_, '_>,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, RenderMagicNumber>>>,
) {
    let mut mesh: Mesh = Plane3d::new(Vec3::Z, Vec2::new(20., 20.)).mesh().into();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]],
    );
    mesh.insert_attribute(ATTR_MAGIC_NUMBER1, vec![1.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER2, vec![0.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER3, vec![0.5; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER4, vec![1.; 4]);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color_texture: Some(server.load("ferris.png")),
                ..Default::default()
            },
            extension: RenderMagicNumber {},
        })),
    ));

    let mut mesh: Mesh = Plane3d::new(Vec3::Z, Vec2::new(20., 20.)).mesh().into();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]],
    );
    mesh.insert_attribute(ATTR_MAGIC_NUMBER1, vec![1.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER2, vec![0.5; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER3, vec![0.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER4, vec![1.; 4]);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color_texture: Some(server.load("ferris.png")),
                ..Default::default()
            },
            extension: RenderMagicNumber {},
        })),
        Transform::from_xyz(-50., 0., 0.),
    ));

    let mut mesh: Mesh = Plane3d::new(Vec3::Z, Vec2::new(20., 20.)).mesh().into();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]],
    );
    mesh.insert_attribute(ATTR_MAGIC_NUMBER1, vec![0.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER2, vec![1.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER3, vec![1.; 4]);
    mesh.insert_attribute(ATTR_MAGIC_NUMBER4, vec![1.; 4]);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color_texture: Some(server.load("ferris.png")),
                ..Default::default()
            },
            extension: RenderMagicNumber {},
        })),
        Transform::from_xyz(50., 0., 0.),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(100.0, 100.0, 150.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    ));
}
