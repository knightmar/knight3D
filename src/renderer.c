#include "renderer.h"
#include "shader.h" // New header for shader functions
#include <cglm/cglm.h> // Include cglm main header
#include <math.h> // For M_PI if cglm doesn't provide it directly or for other math

#ifndef M_PI // cglm might define its own or use math.h's
    #define M_PI 3.14159265358979323846
#endif

static SDL_Window *window = NULL;
static SDL_GLContext gl_context;
static SDL_Renderer *renderer = NULL;

// VAO and VBO handles (Removed obsolete globals for single triangles)
// static GLuint vao;
// static GLuint vbo_vertices;
// static GLuint vbo_colors;
static GLuint shader_program; // Keep shader program global
static GLint mvp_location;    // Keep MVP location global
static GLint useOverrideColor_location = -1; // Location for shader uniform
static GLint overrideColor_location = -1;    // Location for shader uniform


const int targetFPS = 60;
const int frameDelay = 1000 / targetFPS;

Uint32 frameStart;
int frameTime;

void initialize_renderer(void (*setup)(void)) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL Init Error: %s\n", SDL_GetError());
        exit(1);
    }

    // Set OpenGL version to 3.3 core profile
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);

    window = SDL_CreateWindow("3D Example", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 
                             1200, 1200, SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN);
    if (!window) {
        fprintf(stderr, "Window creation failed: %s\n", SDL_GetError());
        SDL_Quit();
        exit(1);
    }

    gl_context = SDL_GL_CreateContext(window);
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetSwapInterval(1);

    // Initialize GLEW
    glewExperimental = GL_TRUE;
    if (glewInit() != GLEW_OK) {
        fprintf(stderr, "Failed to initialize GLEW\n");
        exit(1);
    }

    // Create VAO and VBOs (Removed obsolete globals for single triangles)
    // glGenVertexArrays(1, &vao);
    // glGenBuffers(1, &vbo_vertices);
    // glGenBuffers(1, &vbo_colors);

    // Load shaders
    shader_program = load_shaders("src/shaders/vertex.glsl", "src/shaders/fragment.glsl"); // Use static global
    glUseProgram(shader_program);

    // Get MVP matrix uniform location
    mvp_location = glGetUniformLocation(shader_program, "MVP"); // Use static global
    // Get override color uniform locations
    useOverrideColor_location = glGetUniformLocation(shader_program, "useOverrideColor");
    overrideColor_location = glGetUniformLocation(shader_program, "overrideColor");

    // Basic check for uniform locations
    if (mvp_location == -1 || useOverrideColor_location == -1 || overrideColor_location == -1) {
        fprintf(stderr, "Renderer Init: Failed to get uniform locations!\n");
        // Consider exiting or handling error
    }


    glEnable(GL_DEPTH_TEST);
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

    setup();
}

// Removed obsolete render_triangle function
// void render_triangle(const TRIANGLE *triangle) { ... }

void main_loop(void (*draw)(void)) {
    bool running = true;
    SDL_Event event;

    while (running) {
        frameStart = SDL_GetTicks();

        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // --- MVP MATRIX CALCULATION using cglm ---
        mat4 projection_matrix, view_matrix, model_matrix, mvp_matrix;

        // Projection matrix
        float near_plane = 0.1f, far_plane = 100.0f, fov_deg = 45.0f;
        float aspect_ratio = 1200.0f / 1200.0f; // window_width / window_height
        glm_perspective(glm_rad(fov_deg), aspect_ratio, near_plane, far_plane, projection_matrix);

        // View matrix (camera)
        vec3 eye    = {0.0f, 0.0f, 10.0f}; // Camera position
        vec3 center = {0.0f, 0.0f, 0.0f};  // Look at point
        vec3 up     = {0.0f, 1.0f, 0.0f};  // Up vector
        glm_lookat(eye, center, up, view_matrix);

        // Model matrix (identity for now, object at origin)
        // Model matrix (make it rotate over time)
        float angle = SDL_GetTicks() * 0.0005f; // Radians per millisecond (adjust speed)
        glm_rotate_make(model_matrix, angle, (vec3){0.0f, 1.0f, 0.0f}); // Rotate around Y axis
        // glm_mat4_identity(model_matrix); // Keep object at origin, only rotate


        // MVP = Projection * View * Model
        mat4 view_model;
        glm_mat4_mul(view_matrix, model_matrix, view_model);
        glm_mat4_mul(projection_matrix, view_model, mvp_matrix);
        
        glUniformMatrix4fv(mvp_location, 1, GL_FALSE, (const GLfloat*)mvp_matrix);

        if (draw != NULL)
            draw();

        SDL_GL_SwapWindow(window);
    frameTime = SDL_GetTicks() - frameStart;

    if (frameDelay > frameTime) {
        SDL_Delay(frameDelay - frameTime);
    }
    // Print frame time every ~second to check performance
    static Uint32 lastPrintTime = 0;
    if (SDL_GetTicks() - lastPrintTime > 1000) {
        printf("Frame Time: %d ms\n", frameTime);
        lastPrintTime = SDL_GetTicks();
    }
    }
}

void cleanup_renderer() {
    // It's good practice to detach and delete shader program
    if (shader_program != 0) { // Check if shader_program is valid
        glDeleteProgram(shader_program);
    }
    // Removed deletion of obsolete global VAO/VBOs
    // glDeleteVertexArrays(1, &vao);
    // glDeleteBuffers(1, &vbo_vertices);
    // glDeleteBuffers(1, &vbo_colors);
    SDL_GL_DeleteContext(gl_context);
    SDL_DestroyWindow(window);
    SDL_Quit();
}

// Sets the override color uniforms in the shader
void renderer_set_override_color(bool enable, float r, float g, float b) {
    // Ensure shader program is active (optional, could assume it is)
    // glUseProgram(shader_program); 

    if (useOverrideColor_location != -1) {
        glUniform1i(useOverrideColor_location, enable ? 1 : 0); // Use 1 for true, 0 for false for GLint uniform bool
    }
    if (overrideColor_location != -1 && enable) {
        glUniform3f(overrideColor_location, r, g, b);
    }
}
