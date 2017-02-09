#version 110

varying vec3 v_normal;

const vec3 LIGHT = vec(-0.2, 0.8, 0.1);

void main() {
  float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
  vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
  gl_FragColor = vec4(color, 1.0);
}
