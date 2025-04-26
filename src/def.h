#ifndef DEF_H
#define DEF_H

#include <GL/gl.h>


typedef struct {
    unsigned char r, g, b;
} COLOR;

struct triangle {
    GLfloat points[3][3];
    COLOR colors[3];
};

#define RED (COLOR){255, 0, 0}
#define GREEN (COLOR){0, 255, 0}
#define BLUE (COLOR){0, 0, 255}
#define WHITE (COLOR){255, 255, 255}
#define BLACK (COLOR){0, 0, 0}
#define YELLOW (COLOR){255, 255, 0}
#define CYAN (COLOR){0, 255, 255}
#define MAGENTA (COLOR){255, 0, 255}
#define ORANGE (COLOR){255, 165, 0}
#define PURPLE (COLOR){128, 0, 128}
#define PINK (COLOR){255, 192, 203}

typedef struct {
    float points[3][3];
    COLOR colors[3];
} triangle;

COLOR *generate_colors(COLOR color);

struct triangle default_triangle();

#endif // DEF_H
