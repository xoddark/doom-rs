#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;
layout (location = 3) in float light;

out TVertexData {
	float light;
	vec3 normal;
	vec2 uv;
} outData;

uniform mat4 view;
uniform mat4 proj;

void main() {
    vec4 newPos = vec4(position.x, position.y, position.z, 1.0);
	gl_Position = proj * view * newPos;
    outData.light = light;
	outData.normal = normal;
	outData.uv = uv;
}