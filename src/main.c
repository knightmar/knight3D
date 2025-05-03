#include "utils/colors.h"
#include "objects/triangle.h"
#include "renderer.h"
#include "objects/shape.h"
#include "obj_files/obj_parser.h"




void hand_points() {
    Point p[] = {
        {1.7410, -0.0000, 0.3113},
        {1.5342, 0.7715, 0.3113},
        {0.9695, 1.3363, 0.3113},
        {0.1979, 1.5430, 0.3113},
        {-0.5736, 1.3363, 0.3113},
        {-1.1384, 0.7715, 0.3113},
        {-1.3451, -0.0000, 0.3113}
    };

    SHAPE a = create_shape(p, sizeof(p) / sizeof(Point), RED, BLACK);
}
SHAPE *shape = NULL;

void update() {
    render_shape(shape);
}

void setup() {
    OBJ_FILE *obj = read_obj_file("/home/knightmar/code/sdl3d/ressources/IS.obj");

    shape = &obj->shape;
}


int main() {
    initialize_renderer(&setup);
    main_loop(&update);
    cleanup_renderer();
    return 0;
}
