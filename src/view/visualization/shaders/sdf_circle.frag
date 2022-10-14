#version 330

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

float circle(vec2 pos, float radius)
{
    float r = length(pos) - radius;
	return r;
}

void main()
{
    vec2 uv = texcoord;
    uv -= 0.5; // This moves origin to the center
    float c = sign(circle(uv, wavepoint));
    c = 1 - c;

    fragColor = vec4(c, c, c, c);
}
