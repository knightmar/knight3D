#include <time.h>

#include "utils/colors.h"
#include "objects/triangle.h"
#include "renderer.h"
#include "objects/shape.h"

void update() {
    Point pyramid_points[] = {
        {0, 0, 0},
        {0.2, 0, 0.2},
        {0, 0.2, 0},
    };
    SHAPE shape1 = create_shape(pyramid_points, 6, BLUE, YELLOW);
    triangulate_and_render(&shape1);
}

int main(int argc, char *argv[]) {
    initialize_renderer();
    initialize_triangle_rendering(); // Initialisation des VAO/VBO
    main_loop(&update);
    cleanup_renderer();
    return 0;
}
