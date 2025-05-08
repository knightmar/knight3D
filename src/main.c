// Include standard headers first
#include <stdio.h>  // For printf/fprintf
#include <stdlib.h> // For exit, free

// Include project headers - ORDER MATTERS
#include "utils/colors.h" // Define COLOR first
#include "objects/point.h"  // Define Point (needed by shape/parser)
#include "obj_files/obj_parser.h" // Uses Point, COLOR
#include "objects/shape.h"        // Uses Point, COLOR, GLuint
#include "renderer.h"             // Uses setup/draw callbacks

// #include "objects/triangle.h" // Removed

// Global or static storage for our renderable model
static SHAPE teapot_model = {0}; // Initialize to zero

// The drawing function called by the main loop
void update() {
    // Render the teapot model using its VAO and VBOs
    render_shape_batched(&teapot_model);
}

// One-time setup function called by the renderer initialization
void setup() {
    // 1. Load OBJ data from file
    // Using relative path assuming executable runs from project root
    OBJ_Data *obj_data = read_obj_file_data("ressources/TeapotTrue.obj");
    if (!obj_data) {
        fprintf(stderr, "Main Setup: Failed to load OBJ data.\n");
        // Consider how to handle this - maybe exit?
        // For now, we might continue with an empty shape, or exit.
        exit(EXIT_FAILURE); // Exit if model loading fails
    }

    // DEBUG: Print loaded data stats before setting up buffers
    printf("Main Setup: OBJ Data Loaded.\n");
    printf("  Vertices Ptr: %p, Count: %d\n", (void*)obj_data->vertices, obj_data->num_vertices);
    printf("  Colors Ptr:   %p, Count: %d\n", (void*)obj_data->colors, obj_data->num_colors);
    printf("  Indices Ptr:  %p, Count: %d\n", (void*)obj_data->indices, obj_data->num_indices);
    fflush(stdout); // Ensure debug output is flushed before potential crash


    // 2. Setup GPU buffers (VAO/VBOs/EBO) for the shape using the loaded data
    bool success = setup_shape_buffers(&teapot_model,
                                       obj_data->vertices, obj_data->num_vertices,
                                       obj_data->colors, obj_data->num_colors,
                                       obj_data->indices, obj_data->num_indices);

    // 3. Free the CPU-side OBJ data now that it's on the GPU
    // Important: Do this regardless of setup_shape_buffers success if obj_data was allocated.
    if (obj_data->vertices) free(obj_data->vertices); 
    if (obj_data->colors) free(obj_data->colors); 
    if (obj_data->indices) free(obj_data->indices); 
    if (obj_data->name) free(obj_data->name); 
    free(obj_data); // Restore this free
    obj_data = NULL; // Avoid dangling pointer

    if (!success) {
        fprintf(stderr, "Main Setup: Failed to set up GPU buffers for the shape.\n");
        // We freed the obj_data, but buffer creation failed.
        exit(EXIT_FAILURE); // Exit if buffer setup fails
    }

    printf("Main Setup: Teapot model loaded and GPU buffers created.\n");
}


int main() {
    // Initialize SDL, OpenGL context, GLEW, shaders
    initialize_renderer(&setup); // Pass our setup function

    // Start the main rendering loop, passing our drawing function
    main_loop(&update);

    // Cleanup GPU resources for the shape
    cleanup_shape_buffers(&teapot_model);

    // Cleanup SDL and OpenGL context
    cleanup_renderer();

    return 0;
}
