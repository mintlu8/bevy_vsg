import bevy_sprite_render::mesh2d::{
    functions as mesh_functions,
    view_bindings::view,
    vertex_output::VertexOutput as StdVertexOutput,
};

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @if(VERTEX_POSITIONS)
    @location(0) position: vec3<f32>,
    @if(VERTEX_NORMALS)
    @location(1) normal: vec3<f32>,
    @if(VERTEX_UVS)
    @location(2) uv: vec2<f32>,
    @if(VERTEX_TANGENTS)
    @location(3) tangent: vec4<f32>,
    @if(VERTEX_COLORS)
    @location(4) color: vec4<f32>,
    @location(20) magic_number: f32,

};

struct MaybeCompressedVertex {
    @builtin(instance_index) instance_index: u32,
    @if(VERTEX_POSITIONS && VERTEX_POSITIONS_COMPRESSED)
    @location(0) compressed_position: vec4<f32>,
    @if(VERTEX_POSITIONS && !VERTEX_POSITIONS_COMPRESSED)
    @location(0) position: vec3<f32>,
    @if(VERTEX_NORMALS && VERTEX_NORMALS_COMPRESSED)
    @location(1) compressed_normal: vec2<f32>,
    @if(VERTEX_NORMALS && !VERTEX_NORMALS_COMPRESSED)
    @location(1) normal: vec3<f32>,
    @if(VERTEX_UVS && VERTEX_UVS_COMPRESSED)
    @location(2) compressed_uv: vec2<f32>,
    @if(VERTEX_UVS && !VERTEX_UVS_COMPRESSED)
    @location(2) uv: vec2<f32>,
    @if(VERTEX_TANGENTS && VERTEX_TANGENTS_COMPRESSED)
    @location(3) compressed_tangent: vec2<f32>,
    @if(VERTEX_TANGENTS && !VERTEX_TANGENTS_COMPRESSED)
    @location(3) tangent: vec4<f32>,
    @if(VERTEX_COLORS)
    @location(4) color: vec4<f32>,
    @location(20) magic_number: f32,

};

struct VertexOutput {
    /// this is `clip position` when the struct is used as a vertex stage output
    /// and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @if(VERTEX_TANGENTS)
    @location(3) world_tangent: vec4<f32>,
    @if(VERTEX_COLORS)
    @location(4) color: vec4<f32>,
    @location(5) @interpolate(flat) instance_index: u32,
    @location(20) magic_number: f32,

};

/// The instance_index parameter must match vertex_in.instance_index. This is a work around for a wgpu dx12 bug.
/// See https://github.com/gfx-rs/naga/issues/2416
fn decompress_vertex(vertex_in: MaybeCompressedVertex, instance_index: u32) -> Vertex {
    let mesh_metadata = mesh_functions::get_metadata(instance_index);
    var uncompressed_vertex: Vertex;
    uncompressed_vertex.instance_index = instance_index;
    @if(VERTEX_POSITIONS_COMPRESSED)
    uncompressed_vertex.position = bevy_render::utils::decompress_vertex_position(vertex_in.compressed_position, mesh_metadata.aabb_center, mesh_metadata.aabb_half_extents);
    @else
    uncompressed_vertex.position = vertex_in.position;
    @if(VERTEX_NORMALS && VERTEX_NORMALS_COMPRESSED)
    uncompressed_vertex.normal = bevy_render::utils::decompress_vertex_normal(vertex_in.compressed_normal);
    @if(VERTEX_NORMALS && !VERTEX_NORMALS_COMPRESSED)
    uncompressed_vertex.normal = vertex_in.normal;
    @if(VERTEX_UVS && VERTEX_UVS_COMPRESSED)
    let uv_min_and_extents = mesh_metadata.uv_channels_min_and_extents[0];
    @if(VERTEX_UVS && VERTEX_UVS_COMPRESSED)
    uncompressed_vertex.uv = bevy_render::utils::decompress_vertex_uv(vertex_in.compressed_uv, uv_min_and_extents);
    @if(VERTEX_UVS && !VERTEX_UVS_COMPRESSED)
    uncompressed_vertex.uv = vertex_in.uv;
    @if(VERTEX_TANGENTS && VERTEX_TANGENTS_COMPRESSED)
    uncompressed_vertex.tangent = bevy_render::utils::decompress_vertex_tangent(vertex_in.compressed_tangent);
    @if(VERTEX_TANGENTS && !VERTEX_TANGENTS_COMPRESSED)
    uncompressed_vertex.tangent = vertex_in.tangent;
    @if(VERTEX_COLORS)
    uncompressed_vertex.color = vertex_in.color;
    uncompressed_vertex.magic_number = vertex_in.magic_number;

    return uncompressed_vertex;
}

fn into_std_vertex(in: VertexOutput) -> StdVertexOutput {
    var out: StdVertexOutput;
    out.position = in.position;
    out.world_position = in.world_position;
    out.world_normal = in.world_normal;
    out.uv = in.uv;
    @if(VERTEX_TANGENTS)
    out.world_tangent = in.world_tangent;
    @if(VERTEX_COLORS)
    out.color = in.color;
    out.instance_index = in.instance_index;
    return out;
}

@vertex
fn vertex(vertex: MaybeCompressedVertex) -> VertexOutput {
    var out: VertexOutput;
    let _vert0 = decompress_vertex(vertex, vertex.instance_index);
    let _vert1 = _vert0;
    let uncompressed_vertex = _vert1;
@if(VERTEX_UVS)
    out.uv = uncompressed_vertex.uv;

@if(VERTEX_POSITIONS)
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
@if(VERTEX_POSITIONS)
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        world_from_local,
        vec4<f32>(uncompressed_vertex.position, 1.0)
    );
@if(VERTEX_POSITIONS)
    out.position = mesh_functions::mesh2d_position_world_to_clip(out.world_position);

@if(VERTEX_NORMALS)
    out.world_normal = mesh_functions::mesh2d_normal_local_to_world(uncompressed_vertex.normal, vertex.instance_index);

@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh2d_tangent_local_to_world(
        world_from_local,
        uncompressed_vertex.tangent
    );

@if(VERTEX_COLORS)
    out.color = uncompressed_vertex.color;

    out.instance_index = vertex.instance_index;

    out.magic_number = vertex.magic_number;

    let out2 = out;
    return out;
}
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var texture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var texture_sampler: sampler;
@fragment
fn fragment(
    input: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = textureSample(texture, texture_sampler, input.uv);
    if input.magic_number == 0.0 {
        color.g = 0.0;
        color.b = 0.0;
    }
    if input.magic_number == 1.0 {
        color.r = 0.0;
        color.b = 0.0;
    }
    if input.magic_number == 2.0 {
        color.r = 0.0;
        color.g = 0.0;
    }
    return color;
}
