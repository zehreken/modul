#version 330

uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

void main()
{
    vec2 uv = texcoord;
    
    float sign = step(uv.y - wavepoint * 10.0, 0.0); // First parameter is 'edge'
    float brightness = clamp(0.0, 0.1, wavepoint) * 8.0 + 0.2;
    fragColor = vec4(
        0.99 * brightness,
        1.0 * brightness * brightness,
        1.0 - brightness,
        1.0
    ) * sign;
}