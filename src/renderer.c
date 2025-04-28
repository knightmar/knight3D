
#include "renderer.h"

#include "objects/shape.h"


GLuint vao, vbo;
static SDL_Window *window = NULL;
static SDL_GLContext gl_context;

void initialize_renderer() {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL Init Error: %s\n", SDL_GetError());
        exit(1);
    }

    window = SDL_CreateWindow("3D Example", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 1200, 1200,
                              SDL_WINDOW_OPENGL);
    if (!window) {
        fprintf(stderr, "Window creation failed: %s\n", SDL_GetError());
        SDL_Quit();
        exit(1);
    }

    gl_context = SDL_GL_CreateContext(window);
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetSwapInterval(1);

    glewExperimental = GL_TRUE;
    if (glewInit() != GLEW_OK) {
        fprintf(stderr, "Erreur lors de l'initialisation de GLEW\n");
        exit(1);
    }

    glEnable(GL_DEPTH_TEST);
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
}

void initialize_triangle_rendering() {
    glGenVertexArrays(1, &vao);
    glGenBuffers(1, &vbo);

    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);

    glEnableVertexAttribArray(0); // Position
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(float) * 6, (void *)0);

    glEnableVertexAttribArray(1); // Couleur
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(float) * 6, (void *)(sizeof(float) * 3));

    glBindVertexArray(0);
}

void render_triangle(const TRIANGLE *triangle) {
    float vertices[18];
    for (int i = 0; i < 3; i++) {
        vertices[i * 6 + 0] = triangle->points[i].x;
        vertices[i * 6 + 1] = triangle->points[i].y;
        vertices[i * 6 + 2] = triangle->points[i].z;

        vertices[i * 6 + 3] = triangle->colors[i].r / 255.0f;
        vertices[i * 6 + 4] = triangle->colors[i].g / 255.0f;
        vertices[i * 6 + 5] = triangle->colors[i].b / 255.0f;
    }

    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);

    glBindVertexArray(vao);
    glDrawArrays(GL_TRIANGLES, 0, 3);
    glBindVertexArray(0);
}



void cleanup_triangle_rendering() {
    glDeleteBuffers(1, &vbo);
    glDeleteVertexArrays(1, &vao);
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

        draw();

        SDL_GL_SwapWindow(window);
    }
}

void cleanup_renderer() {
    SDL_GL_DeleteContext(gl_context);
    SDL_DestroyWindow(window);
    SDL_Quit();
}
