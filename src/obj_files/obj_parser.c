#include "obj_parser.h"

#include <ctype.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../utils/colors.h"

OBJ_FILE *read_obj_file(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (!file) {
        fprintf(stderr, "Error opening file: %s\n", filename);
        return NULL;
    }

    Point *vertices = NULL;
    int vertex_capacity = 128;
    int vertex_count = 0;
    vertices = malloc(sizeof(Point) * vertex_capacity);

    TRIANGLE *triangles = NULL;
    int triangle_capacity = 256;
    int triangle_count = 0;
    triangles = malloc(sizeof(TRIANGLE) * triangle_capacity);

    char line[512];
    while (fgets(line, sizeof(line), file)) {
        if (line[0] == 'v' && isspace(line[1])) {
            float x, y, z;
            if (sscanf(line, "v %f %f %f", &x, &y, &z) == 3) {
                if (vertex_count >= vertex_capacity) {
                    vertex_capacity *= 2;
                    vertices = realloc(vertices, sizeof(Point) * vertex_capacity);
                }
                vertices[vertex_count++] = (Point){x, y, z};
            }
        } else if (line[0] == 'f' && isspace(line[1])) {
            int indices[32];
            int count = 0;
            char *token = strtok(line + 2, " \t\n\r");
            while (token && count < 32) {
                int idx = atoi(token);
                if (idx < 1 || idx > vertex_count) break;
                indices[count++] = idx - 1;
                token = strtok(NULL, " \t\n\r");
            }

            if (count >= 3) {
                for (int i = 1; i < count - 1; i++) {
                    if (triangle_count >= triangle_capacity) {
                        triangle_capacity *= 2;
                        triangles = realloc(triangles, sizeof(TRIANGLE) * triangle_capacity);
                    }

                    Point a = vertices[indices[0]];
                    Point b = vertices[indices[i]];
                    Point c = vertices[indices[i + 1]];
                    COLOR color = RED;

                    triangles[triangle_count++] = initialize_triangle(
                        (Point[]){a, b, c},
                        (COLOR[]){color, color, color}
                    );
                }
            }
        }
    }

    fclose(file);

    SHAPE shape;
    shape.points = vertices;
    shape.point_count = vertex_count;
    shape.color = RED;
    shape.edge_color = BLACK;
    shape.triangles = triangles;
    shape.triangle_count = triangle_count;

    OBJ_FILE *obj = malloc(sizeof(OBJ_FILE));
    obj->name = "obj";
    obj->shape = shape;
    return obj;
}

