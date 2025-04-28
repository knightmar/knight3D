#ifndef TRIANGLE_H
#define TRIANGLE_H

#include "point.h"

typedef struct {
    unsigned char r, g, b;
} COLOR;

typedef struct {
    Point points[3];
    COLOR colors[3];
} TRIANGLE;

TRIANGLE initialize_triangle(Point points[3], COLOR colors[3]);
void update_triangle(TRIANGLE *triangle, Point points[3], COLOR colors[3]);

float triangle_area(const TRIANGLE *triangle);
#endif // TRIANGLE_H