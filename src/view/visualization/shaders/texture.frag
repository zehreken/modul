#version 330
precision highp float;
in lowp vec2 texcoord;
uniform sampler2D tex;
out vec4 fragColor;

#define C(c) U.x -= 0.5; outColor += char(U, 64 + c)

vec4 char(vec2 p, int c) 
{
    if (p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) return vec4(0, 0, 0, 1e5);
	return textureGrad(tex, p / 16.0 + fract(vec2(c, 15.0 - c / 16.0) / 16.0), dFdx(p / 16.0), dFdy(p / 16.0));
}

void main()
{
    vec2 uv = texcoord;
    uv /= 1.6 * cos(100);
    vec4 outColor = vec4(0.0);

    vec2 position = vec2(0.5);
    float fontSize = 8.0;
    vec2 U = (uv - position) * 64.0 / fontSize;
    C(8);C(5);C(12);C(12);C(15);C(-32);C(23);C(15);C(18);C(12);C(4);C(-31);
    fragColor = outColor;
}