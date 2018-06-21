#version 330 core

uniform vec3 BallColor;
uniform float BallRadius;

in vec2 v_Pos;
out vec4 Target0;

void main() {
    float dist_sqr = dot(v_Pos - BallRadius, v_Pos - BallRadius);
    if (dist_sqr > BallRadius * BallRadius) {
        discard;
    }
    Target0 = vec4(BallColor, 1.0);
}
