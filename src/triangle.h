#ifndef TRIANGLE_H
#define TRIANGLE_H

#include <GL/gl.h>

typedef struct {
    unsigned char r, g, b;
} COLOR;

typedef struct {
    GLfloat points[3][3];
    COLOR colors[3];
} TRIANGLE;

TRIANGLE initialize_triangle(GLfloat points[3][3], COLOR colors[3]);
void update_triangle(TRIANGLE *triangle, GLfloat points[3][3], COLOR colors[3]);

#endif // TRIANGLE_H