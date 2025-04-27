#ifndef RENDERER_H
#define RENDERER_H

#include <SDL2/SDL.h>
#include <GL/gl.h>
#include <stdbool.h>
#include "objects/triangle.h"

void initialize_renderer();
void render_triangle(const TRIANGLE *triangle);
void main_loop(void (*draw)(void));
void cleanup_renderer();

#endif // RENDERER_H