#version 450

layout(location = 0) in vec2 point;
layout(location = 0) out vec2 coordinate;

void main() {
    coordinate = point * 0.5 + 0.5;
    gl_Position = vec4(point, 0.0, 1.0);
}
