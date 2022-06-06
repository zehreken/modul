#version 330
uniform float wavepoint;
in lowp vec2 texcoord;
out vec4 fragColor;

void main()
{
    vec2 uv = texcoord;
    uv -= vec2(1.6, 1.0) * 0.5;
    
    float s = step(uv.y - wavepoint, 0.0);
    float brightness = clamp(0.0, 0.1, wavepoint) * 8.0 + 0.2;
    fragColor = vec4(
        0.5 * s * brightness,
        1.0 * brightness * s * brightness,
        0.0,
        1.0
    );
}