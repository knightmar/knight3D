//
// Created by knightmar on 26/04/25.
//

#ifndef SHAPE_H
#define SHAPE_H
#include "triangle.h"

typedef struct {
    Point *points;
    int point_count;
    COLOR color;
    COLOR edge_color;
} SHAPE;

void triangulate_and_render(SHAPE *shape);

SHAPE create_shape(Point (*points), int point_count, COLOR color, COLOR edge_color);

void draw_edges(SHAPE *shape);

#endif //SHAPE_H
