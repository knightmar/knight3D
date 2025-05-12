//
// Created by knightmar on 26/04/25.
//
#include "../renderer.h"
#include "../objects/shape.h"



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

    glUseProgram(shader_program);
    glBindVertexArray(vao);

    // Set uniform matrices for modelview and projection
    glUniformMatrix4fv(modelview_loc, 1, GL_TRUE, modelview_matrix);  // Set the modelview matrix
    glUniformMatrix4fv(projection_loc, 1, GL_TRUE, projection_matrix);  // Set the projection matrix

    glDrawArrays(GL_TRIANGLES, 0, shape->triangle_count * 3);
}


SHAPE create_shape(Point (*points), int point_count, COLOR color, COLOR edge_color) {
    SHAPE shape;
    shape.color = color;
    shape.edge_color = edge_color;
    triangulate(&shape, points, point_count);
    return shape;
}
