#version 460 core

layout (local_size_x = 8, local_size_y = 8) in;

layout (r8, binding = 0) uniform image2D current_generation;

void main() {
    ivec2 pixel_coord = ivec2(gl_GlobalInvocationID.xy);

    vec2 uv = vec2(pixel_coord) / imageSize(current_generation).x;

    imageStore(current_generation, pixel_coord, vec4(uv.yx, .3, 1.));
}
