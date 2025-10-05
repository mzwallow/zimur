// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

// This struct defines the data that the vertex shader will output.
// This data is then passed to the fragment shader.
struct VertexOutput {
    // @builtin(position): This is a special, REQUIRED output. It's the final
    // position of the vertex in "clip space," which the GPU uses to figure
    // out where on the screen to draw it. It must be a 4D vector (x, y, z, w).
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

// @vertex marks this function as the entry point for the vertex shader stage.
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    // We set the required clip_position output. z is 0.0 (no depth),
    // and w is 1.0 (standard for 3D points).
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}


// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

// @fragment marks this function as the entry point for the fragment shader stage.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
