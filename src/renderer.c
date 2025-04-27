#include "renderer.h"

static SDL_Window *window = NULL;
static SDL_GLContext gl_context;

void initialize_renderer() {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL Init Error: %s\n", SDL_GetError());
        exit(1);
    }

    window = SDL_CreateWindow("3D Example", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 800, 600, SDL_WINDOW_OPENGL);
    if (!window) {
        fprintf(stderr, "Window creation failed: %s\n", SDL_GetError());
        SDL_Quit();
        exit(1);
    }

    gl_context = SDL_GL_CreateContext(window);
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetSwapInterval(1);

    glEnable(GL_DEPTH_TEST);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(-1.5, 1.5, -1.5, 1.5, -1.5, 3);
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
}

void render_triangle(const TRIANGLE *triangle) {
    glBegin(GL_TRIANGLES);
    for (int i = 0; i < 3; i++) {
        glColor3ub(triangle->colors[i].r, triangle->colors[i].g, triangle->colors[i].b);
        glVertex3fv(triangle->points[i]);
    }
    glEnd();
}

void main_loop(void (*draw)(void)) {
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

        draw();

        SDL_GL_SwapWindow(window);
    }
}

void cleanup_renderer() {
    SDL_GL_DeleteContext(gl_context);
    SDL_DestroyWindow(window);
    SDL_Quit();
}