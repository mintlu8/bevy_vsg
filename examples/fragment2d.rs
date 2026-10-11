use bevy::{
    mesh::{MeshVertexAttribute, VertexFormat},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

pub static ATTR_MAGIC_NUMBER: MeshVertexAttribute =
    MeshVertexAttribute::new("Magic_Number", 666, VertexFormat::Float32);

pub fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<RenderMagicNumber>::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Clone, AsBindGroup, Asset, Default, TypePath)]
pub struct RenderMagicNumber {
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}


impl Material2d for RenderMagicNumber {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Path("magic_number.wesl".into())
    }

    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("magic_number.wesl".into())
    }

    fn specialize(
        _pipeline: &bevy::sprite_render::Material2dPipeline,
        descriptor: &mut bevy::material::descriptor::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::sprite_render::Material2dKey<Self>,
    ) -> Result<(), bevy::material::specialize::SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[ATTR_MAGIC_NUMBER.at_shader_location(20)])?;
        descriptor.vertex.buffers[0]
            .attributes
            .push(vertex_layout.attributes[0]);
        Ok(())
    }
}

fn setup(
    mut commands: Commands<'_, '_>,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RenderMagicNumber>>,
) {
    let mut mesh: Mesh = Plane3d::new(Vec3::Z, Vec2::new(50., 50.)).mesh().into();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]],
    );
    mesh.insert_attribute(ATTR_MAGIC_NUMBER, vec![0., 0., 0., 0.]);
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(RenderMagicNumber {
            image: server.load("ferris.png"),
        })),
    ));

    let mut mesh: Mesh = Plane3d::new(Vec3::Z, Vec2::new(50., 50.)).mesh().into();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]],
    );
    mesh.insert_attribute(ATTR_MAGIC_NUMBER, vec![1., 1., 1., 1.]);
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(RenderMagicNumber {
            image: server.load("ferris.png"),
        })),
        Transform::from_xyz(150., 0., 0.),
    ));

    let mut mesh: Mesh = Plane3d::new(Vec3::Z, Vec2::new(50., 50.)).mesh().into();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]],
    );
    mesh.insert_attribute(ATTR_MAGIC_NUMBER, vec![2., 2., 2., 2.]);
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(RenderMagicNumber {
            image: server.load("ferris.png"),
        })),
        Transform::from_xyz(-150., 0., 0.),
    ));

    commands.spawn((Camera2d,));
}
