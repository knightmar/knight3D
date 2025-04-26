#include "def.h"
#include "backend.h"


void update() {
    struct triangle t = create_triangle();
    COLOR colors[3] = {
        RED,
        RED,
        BLUE
    };
    set_color(&t, colors);

    draw_triangle(t);

    struct triangle t2 = create_triangle();
    COLOR colors2[3] = {
        BLUE,
        PINK,
        BLUE
    };
    set_color(&t2, colors2);
    set_pos(&t2, (GLfloat[3][3]){
                {0.5f, -0.5f, 0.5f},
                {1.5f, -0.5f, 0.5f},
                {1.0f, 0.5f, 0.5f}
            });

    draw_triangle(t2);
}

int main(int argc, char *argv[]) {
    main_loop(&update);
}
