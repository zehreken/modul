#version 330

in lowp vec2 texcoord;
out vec4 fragColor;

void main()
{
    vec2 uv = texcoord;
    fragColor = vec4(1.0, uv.x, uv.y, 1.0);
}