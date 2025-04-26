#ifndef BACKEND_H
#define BACKEND_H

#include <SDL2/SDL.h>
#include <GL/gl.h>
#include <stdbool.h>
#include <stdio.h>
#include <time.h>
#include "def.h" // Inclure la définition de struct triangle

void main_loop(void (*draw)(void)); // Pointeur vers une fonction
void draw_triangle(const struct triangle t);

struct triangle create_triangle();

void set_color(struct triangle *, COLOR colors[3]);

void set_pos(struct triangle *triangle, GLfloat points[3][3]);


#endif // BACKEND_H
