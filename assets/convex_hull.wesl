import bevy_pbr::render::{
    mesh_bindings::mesh,
    mesh_functions,
    skinning,
    morph::{morph_position, morph_normal, morph_tangent},
    view_transformations::position_world_to_clip,
    forward_io::VertexOutput as StdVertexOutput,
};

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @if(VERTEX_POSITIONS)
    @location(0) position: vec3<f32>,
    @if(VERTEX_NORMALS)
    @location(1) normal: vec3<f32>,
    @if(VERTEX_UVS_A)
    @location(2) uv: vec2<f32>,
    @if(VERTEX_UVS_B)
    @location(3) uv_b: vec2<f32>,
    @if(VERTEX_TANGENTS)
    @location(4) tangent: vec4<f32>,
    @if(VERTEX_COLORS)
    @location(5) color: vec4<f32>,
    @if(SKINNED)
    @location(6) joint_indices: vec4<u32>,
    @if(SKINNED)
    @location(7) joint_weights: vec4<f32>,
    @if(MORPH_TARGETS)
    @builtin(vertex_index) index: u32,
    @location(20) ch_normal: vec3<f32>,

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
    @if(VERTEX_UVS_A && VERTEX_UVS_A_COMPRESSED)
    @location(2) compressed_uv: vec2<f32>,
    @if(VERTEX_UVS_A && !VERTEX_UVS_A_COMPRESSED)
    @location(2) uv: vec2<f32>,
    @if(VERTEX_UVS_B && VERTEX_UVS_B_COMPRESSED)
    @location(3) compressed_uv_b: vec2<f32>,
    @if(VERTEX_UVS_B && !VERTEX_UVS_B_COMPRESSED)
    @location(3) uv_b: vec2<f32>,
    @if(VERTEX_TANGENTS && VERTEX_TANGENTS_COMPRESSED)
    @location(4) compressed_tangent: vec2<f32>,
    @if(VERTEX_TANGENTS && !VERTEX_TANGENTS_COMPRESSED)
    @location(4) tangent: vec4<f32>,
    @if(VERTEX_COLORS)
    @location(5) color: vec4<f32>,
    @if(SKINNED)
    @location(6) joint_indices: vec4<u32>,
    @if(SKINNED)
    @location(7) joint_weights: vec4<f32>,
    @if(MORPH_TARGETS)
    @builtin(vertex_index) index: u32,
    @location(20) ch_normal: vec3<f32>,

};

@if(!MESHLET_MESH_MATERIAL_PASS)

/// The instance_index parameter must match vertex_in.instance_index. This is a work around for a wgpu dx12 bug.
/// See https://github.com/gfx-rs/naga/issues/2416
fn decompress_vertex(vertex_in: MaybeCompressedVertex, instance_index: u32) -> Vertex {
    let mesh_metadata = bevy_pbr::render::mesh_functions::get_metadata(instance_index);
    var uncompressed_vertex: Vertex;
    uncompressed_vertex.instance_index = instance_index;
@if(VERTEX_POSITIONS && VERTEX_POSITIONS_COMPRESSED)
    uncompressed_vertex.position = bevy_render::utils::decompress_vertex_position(vertex_in.compressed_position, mesh_metadata.aabb_center, mesh_metadata.aabb_half_extents);
@if(VERTEX_POSITIONS && !VERTEX_POSITIONS_COMPRESSED)
    uncompressed_vertex.position = vertex_in.position;
@if(VERTEX_NORMALS && VERTEX_NORMALS_COMPRESSED)
    uncompressed_vertex.normal = bevy_render::utils::decompress_vertex_normal(vertex_in.compressed_normal);
@if(VERTEX_NORMALS && !VERTEX_NORMALS_COMPRESSED)
    uncompressed_vertex.normal = vertex_in.normal;
@if(VERTEX_UVS_A && VERTEX_UVS_A_COMPRESSED) {
    let uv_min_and_extents_a = mesh_metadata.uv_channels_min_and_extents[0];
    uncompressed_vertex.uv = bevy_render::utils::decompress_vertex_uv(vertex_in.compressed_uv, uv_min_and_extents_a);
}
@if(VERTEX_UVS_A && !VERTEX_UVS_A_COMPRESSED)
    uncompressed_vertex.uv = vertex_in.uv;
@if(VERTEX_UVS_B && VERTEX_UVS_B_COMPRESSED) {
    let uv_min_and_extents_b = mesh_metadata.uv_channels_min_and_extents[1];
    uncompressed_vertex.uv_b = bevy_render::utils::decompress_vertex_uv(vertex_in.compressed_uv_b, uv_min_and_extents_b);
}
@if(VERTEX_UVS_B && !VERTEX_UVS_B_COMPRESSED)
    uncompressed_vertex.uv_b = vertex_in.uv_b;
@if(VERTEX_TANGENTS && VERTEX_TANGENTS_COMPRESSED)
    uncompressed_vertex.tangent = bevy_render::utils::decompress_vertex_tangent(vertex_in.compressed_tangent);
@if(VERTEX_TANGENTS && !VERTEX_TANGENTS_COMPRESSED)
    uncompressed_vertex.tangent = vertex_in.tangent;
@if(VERTEX_COLORS)
    uncompressed_vertex.color = vertex_in.color;
@if(SKINNED) {
    uncompressed_vertex.joint_indices = vertex_in.joint_indices;
    uncompressed_vertex.joint_weights = vertex_in.joint_weights;
}
@if(MORPH_TARGETS)
    uncompressed_vertex.index = vertex_in.index;
    uncompressed_vertex.ch_normal = vertex_in.ch_normal;

    return uncompressed_vertex;
}

struct VertexOutput {
    /// This is `clip position` when the struct is used as a vertex stage output
    /// and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @if(VERTEX_UVS_A)
    @location(2) uv: vec2<f32>,
    @if(VERTEX_UVS_B)
    @location(3) uv_b: vec2<f32>,
    @if(VERTEX_TANGENTS)
    @location(4) world_tangent: vec4<f32>,
    @if(VERTEX_COLORS)
    @location(5) color: vec4<f32>,
    @if(VERTEX_OUTPUT_INSTANCE_INDEX)
    @location(6) @interpolate(flat) instance_index: u32,
    @if(VISIBILITY_RANGE_DITHER)
    @location(7) @interpolate(flat) visibility_range_dither: i32,

}


@if(MORPH_TARGETS)
/// The instance_index parameter must match vertex_in.instance_index. This is a work around for a wgpu dx12 bug.
/// See https://github.com/gfx-rs/naga/issues/2416
fn morph_vertex(vertex_in: Vertex, instance_index: u32) -> Vertex {
    var vertex = vertex_in;
    let first_vertex = mesh[instance_index].first_vertex_index;
    let vertex_index = vertex.index - first_vertex;
    let morph_descriptor_index = mesh[instance_index].morph_descriptor_index;

    let weight_count = bevy_pbr::render::morph::layer_count(morph_descriptor_index);
    for (var i: u32 = 0u; i < weight_count; i ++) {
        let weight = bevy_pbr::render::morph::weight_at(i, morph_descriptor_index);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph_position(vertex_index, i, morph_descriptor_index);
        @if(VERTEX_NORMALS)
        vertex.normal += weight * morph_normal(vertex_index, i, morph_descriptor_index);
        @if(VERTEX_TANGENTS)
        vertex.tangent += vec4(weight * morph_tangent(vertex_index, i, morph_descriptor_index), 0.0);
    }
    return vertex;
}

fn into_std_vertex(in: VertexOutput) -> StdVertexOutput {
    var out: StdVertexOutput;
    out.position = in.position;
    out.world_position = in.world_position;
    out.world_normal = in.world_normal;
    @if(VERTEX_UVS_A)
    out.uv = in.uv;
    @if(VERTEX_UVS_B)
    out.uv_b = in.uv_b;
    @if(VERTEX_TANGENTS)
    out.world_tangent = in.world_tangent;
    @if(VERTEX_COLORS)
    out.color = in.color;
    @if(VERTEX_OUTPUT_INSTANCE_INDEX)
    out.instance_index = in.instance_index;
    @if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = in.visibility_range_dither;
    return out;
}

@vertex
fn vertex(vertex_no_morph: MaybeCompressedVertex) -> VertexOutput {
    var out: VertexOutput;
    let _vert0 = decompress_vertex(vertex_no_morph, vertex_no_morph.instance_index);
    let _vert1 = preprocess(_vert0);
@if(MORPH_TARGETS)
    var vertex = morph_vertex(_vert1, vertex_no_morph.instance_index);
@else
    var vertex = _vert1;

    let mesh_world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);

@if(SKINNED)
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var world_from_local = skinning::skin_model(
        vertex.joint_indices,
        vertex.joint_weights,
        vertex_no_morph.instance_index
    );
@else
    var world_from_local = mesh_world_from_local;

@if(VERTEX_NORMALS && SKINNED)
    out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
@if(VERTEX_NORMALS && !SKINNED)
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex_no_morph.instance_index
    );

@if(VERTEX_POSITIONS)
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
@if(VERTEX_POSITIONS)
    out.position = position_world_to_clip(out.world_position.xyz);

@if(VERTEX_UVS_A)
    out.uv = vertex.uv;
@if(VERTEX_UVS_B)
    out.uv_b = vertex.uv_b;

@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex_no_morph.instance_index
    );

@if(VERTEX_COLORS)
    out.color = vertex.color;

@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = vertex_no_morph.instance_index;

@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex_no_morph.instance_index, mesh_world_from_local[3]);


    let out2 = out;
    return out2;
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> size: f32;
fn preprocess(input: Vertex) -> Vertex {
    var result = input;
    result.position += result.ch_normal * size;
    return result;
}
