#version 330
precision highp float;
in lowp vec2 texcoord;
uniform sampler2D tex;
uniform int columnCount = 16;
out vec4 fragColor;

#define C(c) U.x -= 1.0; outColor += char(U, 0 + c)

vec4 char(vec2 p, int c) 
{
    if (p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) return vec4(0, 0, 0, 1e5);
	return textureGrad(tex, p / columnCount + fract(vec2(c, (columnCount - 1) - c / columnCount) / columnCount), dFdx(p / 16.0), dFdy(p / 16.0));
}

void main()
{
    vec2 uv = vec2(texcoord.s, 1.0 - texcoord.t);
    
    vec4 outColor = vec4(0.0);

    vec2 position = vec2(0.5);
    float fontSize = 8.0;
    vec2 U = (uv - position) * 64.0 / fontSize;
    C(166);C(180);C(210);
    fragColor = outColor;
}