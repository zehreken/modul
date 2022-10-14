#version 330

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

float circle(vec2 pos, float radius)
{
    float r = sqrt(pow(pos.x, 2.0) + pow(pos.y, 2.0)) - radius;
	return r;
}

void main()
{
    vec2 uv = texcoord;
    uv -= 0.5; // This moves origin to the center

    vec2 eye = vec2(uv.x, uv.y * clamp((100.0 - pow(wavepoint, 0.5) * 999.0), 4.0, 100.0));
    vec3 eye_c = vec3(1.0 - sign(circle(eye, 0.4)), 0.0, 0.0);

    vec2 iris = uv;
    vec3 iris_c = vec3(0.0, 1.0 - sign(circle(iris, 0.08)), 0.0);

    vec3 color = eye_c + iris_c;
    color.y *= eye_c.x;

    fragColor = vec4(color, 1.0);
}
