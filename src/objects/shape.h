//
// Created by knightmar on 26/04/25.
// Modified for batch rendering.
//

#ifndef SHAPE_H
#define SHAPE_H

#include <GL/glew.h> // For GLuint, GLsizei
#include "point.h"   // For Point
#include "../utils/colors.h" // For COLOR
#include <stdbool.h> // For bool

// Represents a renderable shape with its own VAO and VBOs for batched rendering
typedef struct {
    GLuint vao;
    GLuint vbo_vertices;
    GLuint vbo_colors;
    GLuint ebo_indices;    // Element Buffer Object for indexed drawing
    GLsizei num_indices;   // Number of indices to draw
} SHAPE;

// Initializes a SHAPE struct's GPU buffers (VAO, VBOs, EBO) with the provided geometry data.
// Data is uploaded with GL_STATIC_DRAW.
// Returns true on success, false on failure.
bool setup_shape_buffers(SHAPE* shape,
                         Point* vertices, int num_vertices,
                         COLOR* colors, int num_colors,        // Assumes num_colors == num_vertices
                         GLuint* indices, int num_indices_val); // Renamed num_indices to avoid conflict

// Renders the shape using its VAO and EBO.
void render_shape_batched(const SHAPE* shape);

// Deletes the GPU buffers (VAO, VBOs, EBO) associated with the shape.
void cleanup_shape_buffers(SHAPE* shape);

#endif //SHAPE_H
