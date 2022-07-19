#version 330

in vec3 pos;
in vec2 uv;
uniform vec3 offset;
out lowp vec2 texcoord;

void main()
{
    gl_Position = vec4(pos + offset, 1);

    texcoord = uv;
    texcoord.x *= 1.6; // Fix aspect ratio
}