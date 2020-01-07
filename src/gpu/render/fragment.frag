#version 450

layout(std140, set = 0, binding = 0) uniform Context {
    float time;
} context;
layout(set = 0, binding = 1) uniform sampler context_sampler;
layout(set = 1, binding = 0) uniform texture2D trade;

layout(location = 0) in vec2 coordinate;
layout(location = 0) out vec4 result;

void main() {
    result = vec4(texture(sampler2D(trade, context_sampler), coordinate).rrr, 1.0);
}
