#include "utils/colors.h"
#include "objects/triangle.h"
#include "renderer.h"
#include "objects/shape.h"

void update() {
    for (int j = 0; j < 10; j++) {
        int i = 2 * j;
        Point cube[] = {
            {-0.3 + i * 0.8, -0.4 + i * 0.12, -0.2 + i * 0.05},
            {-0.7 + i * 1.2, -0.5 + i * 0.09, -0.6 + i * 0.15},
            {-0.1 + i * 0.5, -0.6 + i * 0.2, -0.4 + i * 0.1},
            {-0.2 + i * 1.5, -0.3 + i * 0.1, -0.5 + i * 0.05},
            {-0.6 + i * 0.7, -0.2 + i * 0.18, -0.1 + i * 0.08},
            {-0.4 + i * 1.0, -0.7 + i * 0.05, -0.3 + i * 0.2},
            {-0.5 + i * 0.9, -0.8 + i * 0.11, -0.4 + i * 0.07},
            {-0.2 + i * 1.1, -0.1 + i * 0.13, -0.5 + i * 0.09},
            {-0.8 + i * 0.6, -0.9 + i * 0.04, -0.2 + i * 0.06},
            {-0.3 + i * 1.3, -0.5 + i * 0.07, -0.6 + i * 0.14}
        };
        SHAPE shape1 = create_shape(cube, sizeof(cube) / sizeof(Point), BLUE, YELLOW);
        triangulate_and_render(&shape1);
    }
}


int main(int argc, char *argv[]) {
    initialize_renderer();
    main_loop(&update);
    cleanup_renderer();
    return 0;
}
