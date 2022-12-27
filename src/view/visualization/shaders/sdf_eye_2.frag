#version 330

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

float circle(vec2 pos, float radius)
{
    float r = sqrt(pow(pos.x, 2.0) + pow(pos.y, 2.4)) - radius;
	return r;
}

vec3 getColor(vec3 rgb)
{
    return rgb / 255.0;
}

void main()
{
    vec2 uv = texcoord;
    uv -= 0.5; // This moves origin to the center

    float radius = 1.0 + wavepoint;
    float eye_v = clamp(1.0 - sign(circle(vec2(0.0, 0.0), radius)), 0.0, 1.0);

    float radiusTwo = 0.1 + wavepoint;
    vec2 iris = uv;
    float iris_v = clamp(1.0 - sign(circle(iris, radiusTwo)), 0.0, 1.0);

    float iris_two_v = clamp(1.0 - sign(circle(iris, radiusTwo / 4.0)), 0.0, 1.0);

    iris_v *= eye_v;
    iris_two_v *= eye_v;
    eye_v -= iris_v;
    iris_v -= iris_two_v;

    vec3 eye_color = getColor(vec3(227.0, 227.0, 227.0)) * eye_v;
    vec3 iris_color = getColor(vec3(255.0, 0.0, 55.0)) * iris_v;
    vec3 iris_two_color = getColor(vec3(51.0, 51.0, 51.0)) * iris_two_v;
    vec3 color = eye_color + iris_color + iris_two_color;

    fragColor = vec4(color, 1.0);
}
