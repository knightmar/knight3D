#ifndef RENDERER_H
#define RENDERER_H

#include <GL/glew.h>
#include <SDL2/SDL.h>
#include <GL/gl.h>
#include <stdbool.h>
#include <math.h>
// Removed: #include "objects/triangle.h"

void initialize_renderer(void (*setup)(void));
// Removed: void render_triangle(const TRIANGLE *triangle);
void main_loop(void (*draw)(void));
void cleanup_renderer();

// Sets the override color uniforms in the shader
void renderer_set_override_color(bool enable, float r, float g, float b);

#endif // RENDERER_H
