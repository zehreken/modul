#version 330

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

float box(vec2 pos, vec2 b)
{
    vec2 d = abs(pos) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

void main()
{
    vec2 uv = texcoord;
    uv -= 0.5;
    float c = sign(box(uv, vec2(wavepoint, wavepoint)));

    fragColor = vec4(c, c, c, c);
}