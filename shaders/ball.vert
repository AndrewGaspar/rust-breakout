#version 330 core

uniform vec2 BallMidpoint;
uniform float BallRadius;
in vec2 a_Pos;
out vec2 v_Pos;

void main() {
    v_Pos = a_Pos;
    
    vec2 ballCorner = BallMidpoint - vec2(BallRadius, BallRadius);
    gl_Position = vec4(a_Pos + ballCorner, 0.0, 1.0);
}
