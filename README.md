# bevy_vsg

Command line tool for generating vertex shaders for bevy.

## Example

```text
2d
input:
@location(20) my_attribute: Vec4<f32>,

output:
@location(20) my_attribute: Vec4<f32>,

// Equivalent to input + output + mapping input to output
pipe:
@location(20) my_attribute2: f32,

preprocess:
fn preprocess(vertex: Vertex) -> Vertex {
    ..
}

postprocess:
fn postprocess(input: Vertex, output: VertexOutput, world_from_local: mat4x4<f32>) -> VertexOutput {
    ..
}

fragment:
@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    ..
}
```

## Syntax

First specify either `2d` or `3d`, by default `3d`.

Each line ending with `:` is a command, otherwise input for the previous command.

* `input:`

  Represents custom fields present in vertex shader input but not fragment shader input.

* `output:`

  Represents custom fields present in fragment shader input but not vertex shader input.

* `pipe:`

  Represents custom fields present in both vertex shader and fragment shader input.
  The value is copied into the fragment shader input before the `postprocess` function.

* `preprocess:`
  Optionally define a function that transforms `Vertex` in local space before morph. `Vertex` is a renamed version of
  `UncompressedVertex` with your custom fields.

  ```wgsl
  fn preprocess(vertex: Vertex) -> Vertex {
    ..
  }
  ```

* `postprocess:`
  Optionally define a function that transforms `VertexOutput` after all the standard procedure done
  in the bevy vertex shader.

  ```wgsl
  fn postprocess(input: Vertex, output: VertexOutput, world_from_local: mat4x4<f32>) -> VertexOutput {
      ..
  }
  ```

  `input` is the vertex input after `preprocess` and morph.

* `fragment:`
  Optionally defines a fragment shader.
  A generated function `into_std_vertex` is provided to convert `VertexOutput` into the standard bevy one 
  (imported as `StdVertexOutput`).
  This can be used with functions like `pbr_input_from_standard_material`.

  This is not preferred if `VertexOutput` is not changed as you use a regular `.wesl` file.

## Cli

```sh
bevy_vsg my_shader.txt
```

This can take any file extension except `.wesl` and generates `my_shader.wesl` next to the file.
