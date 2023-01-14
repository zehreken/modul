#version 330

#define INTENSITY 6.0
#define GLOW 2.0

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

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
    vec2 uv = texcoord;
    uv -= 0.5; // This moves origin to the center

    float time = wavepoint * 20.0;
	
	vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
	color.rgb += blob(uv, vec3(0.2, 0.5, 0.0), vec2(1.0, 1.0), vec2(0.05, 0.1), time);
	// color.rgb += blob(uv + 0.1, vec3(0.3, 0.4, 0.0), vec2(0.2, 0.3), vec2(0.3, 0.2), time);
	// color.rgb += blob(uv - 0.2, vec3(0.4, 0.3, 0.0), vec2(-0.3, -0.1), vec2(0.2, 0.3), time);
	// color.rgb += blob(uv + 0.2, vec3(0.5, 0.2, 0.0), vec2(-0.1, -0.2), vec2(0.1, 0.4), time);

	fragColor = color;
}
