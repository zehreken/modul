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
    float radius = 0.1;
    float xPos = mod(texcoord.x, 0.25 * 1.6);
    int radiusIndex = int(texcoord.x / (0.25 * 1.6));
    float c = sign(circle(vec2(xPos, texcoord.y) +
        vec2(-0.125 * 1.6, -0.5), wavepoints[radiusIndex]));

    gl_FragColor = vec4(float(radiusIndex) * 0.25, c, c, 1);
}