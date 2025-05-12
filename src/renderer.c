#include "renderer.h"

#include "objects/shape.h"

static SDL_Window *window = NULL;
static SDL_GLContext gl_context;
static SDL_Renderer *renderer = NULL;

#include <GL/glew.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>



void setup_vao_vbo(SHAPE *shape) {
    if (!shape || !shape->triangles) return;

    glGenVertexArrays(1, &vao);
    glBindVertexArray(vao);

    // Flatten triangle data
    Point *vertices = malloc(sizeof(Point) * shape->triangle_count * 3);
    COLOR *colors = malloc(sizeof(COLOR) * shape->triangle_count * 3);

    for (int i = 0; i < shape->triangle_count; i++) {
        for (int j = 0; j < 3; j++) {
            vertices[i * 3 + j] = shape->triangles[i].points[j];
            colors[i * 3 + j] = shape->triangles[i].colors[j];
        }
    }

    // Upload vertex data
    glGenBuffers(1, &vbo);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(Point) * shape->triangle_count * 3, vertices, GL_STATIC_DRAW);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, (void *)0);
    glEnableVertexAttribArray(0);

    // Upload color data
    glGenBuffers(1, &color_vbo);
    glBindBuffer(GL_ARRAY_BUFFER, color_vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(COLOR) * shape->triangle_count * 3, colors, GL_STATIC_DRAW);
    glVertexAttribPointer(1, 3, GL_UNSIGNED_BYTE, GL_TRUE, 0, (void *)0);
    glEnableVertexAttribArray(1);

    free(vertices);
    free(colors);

    modelview_loc = glGetUniformLocation(shader_program, "u_modelview");
    projection_loc = glGetUniformLocation(shader_program, "u_projection");
}

void initialize_renderer(void (*setup)(void)) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL Init Error: %s\n", SDL_GetError());
        exit(1);
    }

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);

    window = SDL_CreateWindow("3D Example", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 1200, 1200,
                              SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN);
    if (!window) {
        fprintf(stderr, "Window creation failed: %s\n", SDL_GetError());
        SDL_Quit();
        exit(1);
    }

    gl_context = SDL_GL_CreateContext(window);
    SDL_GL_SetSwapInterval(1); // VSync

    GLenum err = glewInit();
    if (err != GLEW_OK) {
        fprintf(stderr, "GLEW Init Failed: %s\n", glewGetErrorString(err));
        exit(1);
    }

    glEnable(GL_DEPTH_TEST);
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

    setup();
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

        // Calculate camera/view/projection
        glm::mat4 model = glm::rotate(glm::mat4(1.0f), SDL_GetTicks() * 0.0005f, glm::vec3(0.0f, 1.0f, 1.0f));
        glm::mat4 view = glm::lookAt(
            glm::vec3(0.0f, 0.0f, 20.0f),
            glm::vec3(0.0f, 0.0f, 0.0f),
            glm::vec3(0.0f, 1.0f, 0.0f)
        );
        glm::mat4 projection = glm::perspective(glm::radians(45.0f), 1.0f, 0.001f, 1000.0f);
        glm::mat4 modelview = view * model;

        glUseProgram(shader_program);
        glUniformMatrix4fv(modelview_loc, 1, GL_FALSE, glm::value_ptr(modelview));
        glUniformMatrix4fv(projection_loc, 1, GL_FALSE, glm::value_ptr(projection));

        if (draw) draw();

        SDL_GL_SwapWindow(window);
    }
}

void cleanup_renderer() {
    SDL_GL_DeleteContext(gl_context);
    SDL_DestroyWindow(window);
    SDL_Quit();
}