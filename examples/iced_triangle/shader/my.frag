#version 450

/*
 An in variable will expect data from outside the shader. In the case of the vertex shader,
 this will come from vertex data. In a fragment shader, an in variable will pull from out variables
 in the vertex shader. When an out variable is defined in the fragment shader, it means that the value
 is meant to be written to a buffer to be used outside the shader program.

 in and out variables can also specify a layout. In shader.frag we specify that the out vec4 f_color should
 be layout(location=0); this means that the value of f_color will be saved to whatever buffer is at location
 zero in our application. In most cases, location=0 is the current texture from the swapchain aka the screen.
*/
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(1.0, 0.0, 0.0, 1.0);
}
