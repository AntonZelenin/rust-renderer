#version 450

void main() {
    const vec2 vertices[3] = vec2[3](
        vec2( 0.25, -0.25),
        vec2(-0.25, -0.25),
        vec2( -0.4, 0.4)
    );
    gl_Position = vec4(vertices[gl_VertexIndex], 0.0, 1.0);
}
