#include "triangle.h"

#include <math.h>

TRIANGLE initialize_triangle(Point points[3], COLOR colors[3]) {
    TRIANGLE t;
    update_triangle(&t, points, colors);
    return t;
}

void update_triangle(TRIANGLE *triangle, Point points[3], COLOR colors[3]) {
    for (int i = 0; i < 3; i++) {
        triangle->colors[i] = colors[i];
        triangle->points[i] = points[i];
    }
}


float triangle_area(const TRIANGLE *triangle) {
    Point A = triangle->points[0];
    Point B = triangle->points[1];
    Point C = triangle->points[2];
    float ABx = B.x - A.x;
    float ABy = B.y - A.y;
    float ABz = B.z - A.z;
    float ACx = C.x - A.x;
    float ACy = C.y - A.y;
    float ACz = C.z - A.z;
    float cross_x = ABy * ACz - ABz * ACy;
    float cross_y = ABz * ACx - ABx * ACz;
    float cross_z = ABx * ACy - ABy * ACx;
    float cross_mag = sqrt(cross_x * cross_x + cross_y * cross_y + cross_z * cross_z);
    return 0.5f * cross_mag;
}