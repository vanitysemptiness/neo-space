struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texcoord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Uniforms {
    resolution: vec2<f32>,
    zoom: f32,
    offset: vec2<f32>,
    grid_color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(input.position, 1.0);
    out.uv = input.texcoord;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = (in.uv * uniforms.resolution + uniforms.offset) / uniforms.zoom;
    let grid = abs(fract(pos - 0.5) - 0.5) / fwidth(pos);
    let line = min(grid.x, grid.y);
    let alpha = 1.0 - min(line, 1.0);
    return vec4<f32>(uniforms.grid_color.rgb, alpha * uniforms.grid_color.a);
}