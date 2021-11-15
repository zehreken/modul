#version 100
precision highp float;
uniform float wavepoint;
varying lowp vec2 texcoord;

void main()
{
    vec2 uv = texcoord;
    uv -= vec2(1.6, 1.0) * 0.5;
    
    float s = sign(wavepoint - uv.y);
    gl_FragColor = vec4(
        1.0 * s,
        0.0,
        0.0,
        1.0
    );
}