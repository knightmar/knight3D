#include "utils/colors.h"
#include "objects/triangle.h"
#include "renderer.h"
#include "objects/shape.h"

void update() {
    Point pyramid_points[] = {
        {0.0f, 1.0f, 0.0f},
        {-1.0f, 0.0f, -1.0f},
        {1.0f, 0.0f, -1.0f},
        {1.0f, 3.0f, 1.0f},
        {-1.0f, 0.0f, 1.0f},
        {-3.0f, 1.0f, -2.0f}
    };
    SHAPE shape1 = create_shape(pyramid_points, 6, BLUE, YELLOW);
    triangulate_and_render(&shape1);
}


int main(int argc, char *argv[]) {
    initialize_renderer();
    main_loop(&update);
    cleanup_renderer();
    return 0;
}
