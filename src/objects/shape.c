//
// Created by knightmar on 26/04/25.
//
#include "../objects/shape.h"

#include <time.h>
#include <stdio.h>
#include <stdlib.h>

#include "../renderer.h"
#include "../utils/colors.h"


extern GLuint vbo, vao;


void triangulate_and_render(SHAPE *shape) {
    if (!shape->points) return;
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
    shape.points = malloc(point_count * sizeof(Point));
    if (!shape.points) {
        fprintf(stderr, "Erreur d'allocation mémoire\n");
        exit(1);
    }
    for (int i = 0; i < point_count; i++) {
        shape.points[i] = points[i];
    }
    shape.point_count = point_count;
    shape.color = color;
    shape.edge_color = edge_color;
    return shape;
}

void draw_edges(SHAPE *shape) {
    if (!shape->points) return;
    float vertices[shape->point_count * shape->point_count * 6];
    int index = 0;

    for (int i = 0; i < shape->point_count; i++) {
        for (int j = i + 1; j < shape->point_count; j++) {
            vertices[index++] = shape->points[i].x;
            vertices[index++] = shape->points[i].y;
            vertices[index++] = shape->points[i].z;
            vertices[index++] = shape->edge_color.r / 255.0f;
            vertices[index++] = shape->edge_color.g / 255.0f;
            vertices[index++] = shape->edge_color.b / 255.0f;

            vertices[index++] = shape->points[j].x;
            vertices[index++] = shape->points[j].y;
            vertices[index++] = shape->points[j].z;
            vertices[index++] = shape->edge_color.r / 255.0f;
            vertices[index++] = shape->edge_color.g / 255.0f;
            vertices[index++] = shape->edge_color.b / 255.0f;
        }
    }

    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);

    glBindVertexArray(vao);
    glDrawArrays(GL_LINES, 0, shape->point_count * 2);
    glBindVertexArray(0);
}
