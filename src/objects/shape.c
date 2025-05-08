//
// Created by knightmar on 26/04/25.
//

#include "../renderer.h"
#include "../objects/shape.h"
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include "../utils/colors.h"

void triangulate(SHAPE *shape, Point *points, int point_count) {
    if (!shape) {
        fprintf(stderr, "Error: Null shape\n");
        return;
    }
    if (!points) {
        fprintf(stderr, "Error: Null points array\n");
        return;
    }
    if (point_count < 3) {
        fprintf(stderr, "Error: Insufficient points to triangulate (%d)\n", point_count);
        return;
    }

    int triangle_count = point_count - 2;
    TRIANGLE *triangles = malloc(sizeof(TRIANGLE) * triangle_count);
    if (!triangles) {
        fprintf(stderr, "Error: Memory allocation failed for triangles\n");
        return;
    }

    COLOR color = shape->color;
    for (int i = 0; i < triangle_count; i++) {
        Point a = points[0];
        Point b = points[i + 1];
        Point c = points[i + 2];

        triangles[i] = initialize_triangle((Point[]){a, b, c}, (COLOR[]){color, color, color});
    }

    shape->triangles = triangles;
    shape->triangle_count = triangle_count;
}

void render_shape(SHAPE *shape) {
    if (!shape || !shape->triangles) return;

    for (int i = 0; i < shape->triangle_count; i++) {
        TRIANGLE triangle = shape->triangles[i];
        render_triangle(&triangle);

        // Removed immediate mode edge drawing:
        // glColor3ub(shape->edge_color.r, shape->edge_color.g, shape->edge_color.b);
        // glBegin(GL_LINES);
        // for (int j = 0; j < 3; j++) {
        //     Point p1 = triangle.points[j];
        //     Point p2 = triangle.points[(j + 1) % 3];
        //     glVertex3f(p1.x, p1.y, p1.z);
        //     glVertex3f(p2.x, p2.y, p2.z);
        // }
        // glEnd();
    }
}

SHAPE create_shape(Point (*points), int point_count, COLOR color, COLOR edge_color) {
    SHAPE shape;
    shape.color = color;
    shape.edge_color = edge_color;
    triangulate(&shape, points, point_count);
    return shape;
}
