#version 330

in vec3 pos;
in vec2 uv;
uniform mat4 mvp;
out lowp vec2 texcoord;

void main()
{
    gl_Position = mvp * vec4(pos, 1.0);

    texcoord = uv;
}