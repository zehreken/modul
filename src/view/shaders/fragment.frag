#version 100
precision highp float;
uniform vec4 wavepoints;
varying lowp vec2 texcoord;

float circle(vec2 pos, float radius)
{
    float r = length(pos) - radius;
	return r;
}

float box(vec2 pos, vec2 b)
{
    vec2 d = abs(pos) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

void main()
{
    vec2 uv = texcoord;
    uv -= vec2(1.6, 1.0) * 0.5;
    float c = sign(circle(vec2(uv.x * 0.25, uv.y), wavepoints[0]));

    gl_FragColor = vec4(c, c, c, 1);
}