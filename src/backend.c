#include "backend.h"
#include "def.h"

void draw_triangle(struct triangle t) {
    for (int i = 0; i < 3; i++) {
        glColor3ub(t.colors[i].r, t.colors[i].g, t.colors[i].b);
        glVertex3fv(t.points[i]);
    }
}

struct triangle create_triangle() {
    struct triangle t = {
        .points = {
            {-1, 1, 1},
            {1, 1, 1},
            {0, -1, 1}
        },
        .colors = {
            {255, 0, 0},
            {0, 255, 0},
            {0, 0, 255}
        }
    };

    return t;
}

void set_color(struct triangle *triangle, COLOR colors[3]) {
    triangle->colors[0] = colors[0];
    triangle->colors[1] = colors[1];
    triangle->colors[2] = colors[2];
}

void set_pos(struct triangle *triangle, GLfloat points[3][3]) {
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            triangle->points[i][j] = points[i][j];
        }
    }
}

void main_loop(void (*draw)(void)) {
    // Modifier le type de draw
    srand(time(NULL));

    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL Init Error: %s\n", SDL_GetError());
        return;
    }

    SDL_Window *window = SDL_CreateWindow("3D Cube Example",
                                          SDL_WINDOWPOS_CENTERED,
                                          SDL_WINDOWPOS_CENTERED,
                                          800, 600,
                                          SDL_WINDOW_OPENGL);

    if (!window) {
        fprintf(stderr, "Window creation failed: %s\n", SDL_GetError());
        SDL_Quit();
        return;
    }

    SDL_GLContext gl_context = SDL_GL_CreateContext(window);
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetSwapInterval(1);

    glEnable(GL_DEPTH_TEST);

    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(-1.5, 1.5, -1.5, 1.5, -1.5, 3);

    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();

    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

    bool running = true;
    SDL_Event event;

    while (running) {
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glLoadIdentity();

        glTranslatef(0.0f, 0.0f, -2.0f);
        glRotatef(SDL_GetTicks() * 0.05f, 1, 1, 1);

        glBegin(GL_TRIANGLES);
        draw(); // Appeler la fonction via le pointeur
        glEnd();

        SDL_GL_SwapWindow(window);
    }

    SDL_GL_DeleteContext(gl_context);
    SDL_DestroyWindow(window);
    SDL_Quit();
}
