#version 100
precision mediump float;

varying vec2 uv;
uniform vec2 resolution;
uniform float zoom;
uniform vec2 offset;
uniform vec4 grid_color;

void main() {
    vec2 pos = (uv * resolution + offset) / zoom;
    vec2 grid = fract(pos);
    float dist = length(grid - 0.5);
    float dot_size = 0.02 / zoom;
    float alpha = 1.0 - smoothstep(dot_size - 0.01, dot_size, dist);
    gl_FragColor = vec4(grid_color.rgb, alpha * grid_color.a);
}