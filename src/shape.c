//
// Created by knightmar on 26/04/25.
//
#include "shape.h"
#include "renderer.h"


void triangulate_and_render(SHAPE *shape) {
    if (shape->point_count < 3) return;

    // Very naive method: "fan triangulation"
    for (int i = 1; i < shape->point_count - 1; i++) {
        TRIANGLE t = initialize_triangle(
            (GLfloat[3][3]){
                { shape->points[0][0], shape->points[0][1], shape->points[0][2] },
                { shape->points[i][0], shape->points[i][1], shape->points[i][2] },
                { shape->points[i+1][0], shape->points[i+1][1], shape->points[i+1][2] }
            },
            (COLOR[3]){ shape->color, shape->color, shape->color }
        );
        render_triangle(&t);
    }
}

SHAPE create_shape(GLfloat (*points)[3], int point_count, COLOR color) {
    SHAPE shape;
    shape.points = points;
    shape.point_count = point_count;
    shape.color = color;
    return shape;
}

