#ifndef RENDERER_H
#define RENDERER_H

#include "../glad/glad.h"
#include <SDL2/SDL.h>
#include <GL/gl.h>
#include <GL/glu.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include "objects/triangle.h"
#include "utils/shaders.h"

GLuint vao, vbo, color_vbo;
GLuint modelview_loc, projection_loc;


void initialize_renderer(void (*setup)(void));
void render_triangle(const TRIANGLE *triangle);
void main_loop(void (*draw)(void));
void cleanup_renderer();

#endif // RENDERER_H