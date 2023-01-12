#version 330

#define INTENSITY 10.5
#define GLOW 4.0

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

float circle(vec2 pos, float radius)
{
    float r = length(pos) - radius;
	return r;
}

vec3 blob(vec2 uv, vec3 color, vec2 speed, vec2 size, float time) {
	vec2 point = vec2(
		sin(speed.x * time) * size.x,
		cos(speed.y * time) * size.y
	);

	float d = 1.0 / distance(uv, point);
	d = pow(d / INTENSITY, GLOW);
	
	return vec3(color.r * d, color.g * d, color.b * d);
}

void main()
{
    /*
    vec2 uv = texcoord;
    uv -= 0.5; // This moves origin to the center
    // float c = sign(circle(uv, 0.05));
    vec3 c = blob(uv, vec3(0.5, 0.0, 0.5), vec2(0.1, 0.1), vec2(0.01, 0.01), wavepoint * 100.0);

    fragColor = vec4(c, 1.0);
    */
    vec2 uv = texcoord;
    uv -= 0.5; // This moves origin to the center

    float time = wavepoint * 10.0;
	
	vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
	color.rgb += blob(uv, vec3(0.2, 0.0, 0.5), vec2(1.7, 2.2), vec2(0.4, 0.1), time);
	color.rgb += blob(uv, vec3(0.3, 0.0, 0.4), vec2(1.2, 2.3), vec2(0.3, 0.2), time);
	color.rgb += blob(uv, vec3(0.4, 0.0, 0.3), vec2(2.3, 2.1), vec2(0.2, 0.3), time);
	color.rgb += blob(uv, vec3(0.5, 0.0, 0.2), vec2(2.1, 1.0), vec2(0.1, 0.4), time);

	fragColor = color;
}
