#include "shape.h"
#include "../renderer.h" // Include renderer for the override color function
#include <stdio.h>  // For fprintf, if needed for errors
#include <stdlib.h> // For NULL

// Initializes a SHAPE struct's GPU buffers (VAO, VBOs, EBO) with the provided geometry data.
// Data is uploaded with GL_STATIC_DRAW.
// Returns true on success, false on failure.
bool setup_shape_buffers(SHAPE* shape,
                         Point* vertices, int num_vertices,
                         COLOR* colors, int num_colors,        // Assumes num_colors == num_vertices
                         GLuint* indices, int num_indices_val) {
    if (!shape || !vertices || !colors || !indices || num_vertices <= 0 || num_colors <= 0 || num_indices_val <= 0) {
        fprintf(stderr, "Shape Setup: Invalid parameters for setting up shape buffers.\n");
        return false;
    }
    if (num_vertices != num_colors) {
        fprintf(stderr, "Shape Setup: Vertex count (%d) and color count (%d) must match.\n", num_vertices, num_colors);
        return false;
    }

    shape->num_indices = num_indices_val;

    // 1. Generate and bind VAO
    glGenVertexArrays(1, &shape->vao);
    if (shape->vao == 0) {
        fprintf(stderr, "Shape Setup: Failed to generate VAO.\n");
        return false;
    }
    glBindVertexArray(shape->vao);

    // 2. Vertex Buffer Object (VBO) for positions
    glGenBuffers(1, &shape->vbo_vertices);
    if (shape->vbo_vertices == 0) {
        fprintf(stderr, "Shape Setup: Failed to generate vertices VBO.\n");
        glDeleteVertexArrays(1, &shape->vao); // Clean up VAO
        shape->vao = 0;
        return false;
    }
    glBindBuffer(GL_ARRAY_BUFFER, shape->vbo_vertices);
    glBufferData(GL_ARRAY_BUFFER, num_vertices * sizeof(Point), vertices, GL_STATIC_DRAW);
    // Attribute location 0: vertexPosition
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(Point), (void*)0);
    glEnableVertexAttribArray(0);

    // 3. Vertex Buffer Object (VBO) for colors
    glGenBuffers(1, &shape->vbo_colors);
     if (shape->vbo_colors == 0) {
        fprintf(stderr, "Shape Setup: Failed to generate colors VBO.\n");
        glDeleteBuffers(1, &shape->vbo_vertices);
        glDeleteVertexArrays(1, &shape->vao);
        shape->vbo_vertices = 0;
        shape->vao = 0;
        return false;
    }
    glBindBuffer(GL_ARRAY_BUFFER, shape->vbo_colors);
    glBufferData(GL_ARRAY_BUFFER, num_colors * sizeof(COLOR), colors, GL_STATIC_DRAW);
    // Attribute location 1: vertexColor
    glVertexAttribPointer(1, 3, GL_UNSIGNED_BYTE, GL_TRUE, sizeof(COLOR), (void*)0); // GL_TRUE to normalize 0-255 to 0.0-1.0
    glEnableVertexAttribArray(1);

    // 4. Element Buffer Object (EBO) for indices
    glGenBuffers(1, &shape->ebo_indices);
    if (shape->ebo_indices == 0) {
        fprintf(stderr, "Shape Setup: Failed to generate EBO.\n");
        glDeleteBuffers(1, &shape->vbo_colors);
        glDeleteBuffers(1, &shape->vbo_vertices);
        glDeleteVertexArrays(1, &shape->vao);
        shape->vbo_colors = 0;
        shape->vbo_vertices = 0;
        shape->vao = 0;
        return false;
    }
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, shape->ebo_indices);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, num_indices_val * sizeof(GLuint), indices, GL_STATIC_DRAW);

    // 5. Unbind VAO (good practice, prevents accidental modification)
    glBindVertexArray(0);
    // Unbind EBO after VAO is unbound, as EBO binding is stored in VAO state
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0); 
    // Unbind Array Buffer
    glBindBuffer(GL_ARRAY_BUFFER, 0);


    printf("Shape Setup: Buffers created successfully. VAO ID: %u, Vertices: %d, Indices: %d\n", shape->vao, num_vertices, shape->num_indices);
    return true;
}

// Renders the shape using its VAO and EBO, optionally drawing edges.
void render_shape_batched(const SHAPE* shape) {
    if (!shape || shape->vao == 0 || shape->num_indices == 0) {
        return; // Nothing to draw
    }

    glBindVertexArray(shape->vao);

    // 1. Draw the filled shape
    renderer_set_override_color(false, 0.0f, 0.0f, 0.0f); // Disable override, use vertex colors
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
    glDrawElements(GL_TRIANGLES, shape->num_indices, GL_UNSIGNED_INT, (void*)0);

    // 2. Draw the edges (wireframe)
    // Enable polygon offset to prevent z-fighting
    glEnable(GL_POLYGON_OFFSET_LINE);
    glPolygonOffset(-1.0f, -1.0f); // Adjust factors as needed

    glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
    renderer_set_override_color(true, 0.0f, 0.0f, 0.0f); // Enable override, use black for edges
    glDrawElements(GL_TRIANGLES, shape->num_indices, GL_UNSIGNED_INT, (void*)0);

    // 3. Reset state
    glDisable(GL_POLYGON_OFFSET_LINE);
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL); // Reset to default fill mode
    renderer_set_override_color(false, 0.0f, 0.0f, 0.0f); // Disable override color

    glBindVertexArray(0);
}

// Deletes the GPU buffers (VAO, VBOs, EBO) associated with the shape.
void cleanup_shape_buffers(SHAPE* shape) {
    if (shape) {
        if (shape->vao != 0) {
            glDeleteVertexArrays(1, &shape->vao);
            shape->vao = 0;
        }
        if (shape->vbo_vertices != 0) {
            glDeleteBuffers(1, &shape->vbo_vertices);
            shape->vbo_vertices = 0;
        }
        if (shape->vbo_colors != 0) {
            glDeleteBuffers(1, &shape->vbo_colors);
            shape->vbo_colors = 0;
        }
        if (shape->ebo_indices != 0) {
            glDeleteBuffers(1, &shape->ebo_indices);
            shape->ebo_indices = 0;
        }
        shape->num_indices = 0;
    }
}
