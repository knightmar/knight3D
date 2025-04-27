//
// Created by knightmar on 26/04/25.
//
#include "../objects/shape.h"

#include <time.h>
#include <stdio.h>
#include <stdlib.h>

#include "../renderer.h"
#include "../utils/colors.h"

void triangulate_and_render(SHAPE *shape) {
    if (shape->point_count < 3) return;

    for (int i = 0; i < shape->point_count - 2; i++) {
        for (int j = i + 1; j < shape->point_count - 1; j++) {
            for (int k = j + 1; k < shape->point_count; k++) {
                Point a = shape->points[i];
                Point b = shape->points[j];
                Point c = shape->points[k];
                COLOR color = shape->color;

                TRIANGLE t = initialize_triangle((Point[]){a, b, c}, (COLOR[]){color, color, color});
                if (triangle_area(&t) > 0.01f) {
                    render_triangle(&t);
                }
            }
        }
    }

    draw_edges(shape);
}

SHAPE create_shape(Point (*points), int point_count, COLOR color, COLOR edge_color) {
    SHAPE shape;
    shape.points = points;
    shape.point_count = point_count;
    shape.color = color;
    shape.edge_color = edge_color;
    return shape;
}

void draw_edges(SHAPE *shape) {
    COLOR color = shape->edge_color;
    for (int i = 0; i < shape->point_count; i++) {
        for (int j = i + 1; j < shape->point_count; j++) {
            glBegin(GL_LINES);
            glColor3ub(color.r, color.g, color.b);
            glVertex3f(shape->points[i].x, shape->points[i].y, shape->points[i].z);
            glVertex3f(shape->points[j].x, shape->points[j].y, shape->points[j].z);
            glEnd();
        }
    }
}
