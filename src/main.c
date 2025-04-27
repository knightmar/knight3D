#include "colors.h"
#include "triangle.h"
#include "renderer.h"
#include "shape.h"

void update() {
    GLfloat pyramid_points[][3] = {
        {0.0f, 1.0f, 0.0f}, // Sommet de la pyramide
        {-1.0f, 0.0f, -1.0f}, // Coin arrière gauche de la base
        {1.0f, 0.0f, -1.0f}, // Coin arrière droit de la base
        {1.0f, 0.0f, 1.0f}, // Coin avant droit de la base
        {-1.0f, 0.0f, 1.0f} // Coin avant gauche de la base
    };
    SHAPE shape1 = create_shape(pyramid_points, 5, RED);
    triangulate_and_render(&shape1);
}


int main(int argc, char *argv[]) {
    initialize_renderer();
    main_loop(&update);
    cleanup_renderer();
    return 0;
}
