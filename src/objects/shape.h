//
// Created by knightmar on 26/04/25.
//

#ifndef SHAPE_H
#define SHAPE_H
#include "triangle.h"

typedef struct {
    TRIANGLE (*triangles);
    Point (*points);
    int triangle_count;
    int point_count;
    COLOR color;
    COLOR edge_color;
} SHAPE;


SHAPE create_shape(Point (*points), int point_count, COLOR color, COLOR edge_color);

void draw_edges(SHAPE *shape);
void triangulate(SHAPE *shape, Point *points, int point_count);
void render_shape(SHAPE *shape);


#endif //SHAPE_H
