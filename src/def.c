#include "def.h"

struct triangle default_triangle() {
    return (struct triangle){
        .points = {
            {-0.5f, -0.5f, 0.5f},
            {0.5f, -0.5f, 0.5f},
            {0.0f, 0.5f, 0.5f}
        },
        .colors = {
            {255, 0, 0},
            {0, 255, 0},
            {0, 0, 255}
        }
    };
}

COLOR *generate_colors(COLOR color) {
    static COLOR colors[3];
    for (int i = 0; i < 3; i++) {
        colors[i] = color;
    }
    return colors;
}
