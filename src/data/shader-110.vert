#version 110

uniform mat4 persp_matrix;
uniform mat4 view_patrix;

attribute vec3 position;
attribute vec3 normal;
varying vec3 v_position;
varying vec3 v_normal;

void main() {
  v_position = position;
  v_normal = normal;
  gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
}
