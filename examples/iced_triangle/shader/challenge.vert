#version 450

layout(location = 0) out vec4 color;

void main() {
    vec2 v1 = vec2(0.4, -0.3);
    vec2 v2 = vec2(-0.8, -0.1);
    vec2 v3 = vec2(-0.4, 0.4);
    const vec2 vertices[3] = vec2[3](v1, v2, v3);
    gl_Position = vec4(vertices[gl_VertexIndex], 0.0, 1.0);
    color = vec4(
        abs(v1[0]) + abs(v1[1]),
        abs(v2[0]) + abs(v2[1]),
        abs(v3[0]) + abs(v3[1]),
        1.0
    );
}
