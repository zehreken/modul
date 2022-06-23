#version 330
precision highp float;
in lowp vec2 texcoord;
uniform sampler2D tex;
uniform int columnCount = 16;
out vec4 fragColor;
uniform float wavepoint;
uniform ivec4 text;

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

    vec2 position = vec2(0.0, 0.4);
    float fontSize = 16.0;
    vec2 U = (uv - position) * 64.0 / fontSize;
    C(int(text.x));C(int(text.y));C(int(text.z));C(int(text.w));
    fragColor = outColor.rrrr;
}

// A 177
// B 178
// C 179
// D 180
// E 181
// F 182
// G 183
// H 184
// I 185
// J 186
// K 187
// L 188
// M 189
// N 190
// O 191
// P 160
// Q 161
// R 162
// S 163
// T 164
// U 165
// V 166
// W 167
// X 168
// Y 169
// Z 170
// . 71