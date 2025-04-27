#include "triangle.h"

TRIANGLE initialize_triangle(GLfloat points[3][3], COLOR colors[3]) {
    TRIANGLE t;
    update_triangle(&t, points, colors);
    return t;
}

void update_triangle(TRIANGLE *triangle, GLfloat points[3][3], COLOR colors[3]) {
    for (int i = 0; i < 3; i++) {
        triangle->colors[i] = colors[i];
        for (int j = 0; j < 3; j++) {
            triangle->points[i][j] = points[i][j];
        }
    }
}
