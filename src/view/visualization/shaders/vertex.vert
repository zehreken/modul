#version 330
in vec2 pos;
in vec2 uv;
uniform vec2 offset;
out lowp vec2 texcoord;

void main()
{
    gl_Position = vec4(pos + offset, 0, 1);

    texcoord = uv;
    texcoord.x *= 1.6; // This is to fix aspect ratio
}