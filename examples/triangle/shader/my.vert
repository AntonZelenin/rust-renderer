#version 450

void main() {
    const vec4 vertices[6] = vec4[6](
        vec4( 0.25, -0.25, 0.5, 1.0),
        vec4(-0.25, -0.25, 0.5, 1.0),
        vec4( -0.4, 0.4, 0.5, 1.0)
    );
    gl_Position = vertices[gl_VertexIndex];
}
