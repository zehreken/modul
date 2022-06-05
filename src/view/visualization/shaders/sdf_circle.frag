#version 100
precision highp float;
uniform float wavepoint;
varying lowp vec2 texcoord;

float circle(vec2 pos, float radius)
{
    float r = length(pos) - radius;
	return r;
}

void main()
{
    vec2 uv = texcoord;
    uv -= vec2(1.6, 1.0) * 0.5;
    float c = sign(circle(vec2(uv.x * 0.25, uv.y), wavepoint));

    gl_FragColor = vec4(c, c, c, 1);
}