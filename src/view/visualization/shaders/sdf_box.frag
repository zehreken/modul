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
    uv -= vec2(1.6, 1.0) * 0.5;
    float c = sign(box(vec2(uv.x * 0.25, uv.y * 0.5), vec2(wavepoint, wavepoint)));

    fragColor = vec4(c, c, c, 1);
}