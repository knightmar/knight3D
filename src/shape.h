//
// Created by knightmar on 26/04/25.
//

#ifndef SHAPE_H
#define SHAPE_H
#include "triangle.h"

typedef struct {
    GLfloat (*points)[3]; // Dynamic array of 3D points
    int point_count; // Number of points
    COLOR color; // Single color for the whole shape
} SHAPE;

void triangulate_and_render(SHAPE *shape);

SHAPE create_shape(GLfloat (*points)[3], int point_count, COLOR color);

#endif //SHAPE_H
