#version 100
precision highp float;
uniform float wavepoint;
varying lowp vec2 texcoord;

void main()
{
    vec2 uv = texcoord;
    uv -= vec2(1.6, 1.0) * 0.5;
    
    float s = step(uv.y - wavepoint, 0.0);
    float brightness = clamp(0.0, 0.1, wavepoint) * 8.0 + 0.2;
    gl_FragColor = vec4(
        1.0 * s * brightness,
        0.5 * brightness * s * brightness,
        0.0,
        1.0
    );
}