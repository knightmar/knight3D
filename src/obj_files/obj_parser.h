//
// Created by knightmar on 02/05/25.
//

#ifndef OBJ_PARSER_H
#define OBJ_PARSER_H

#include <GL/glew.h>           // Include GLEW first for GL types like GLuint
#include "../objects/point.h" // For Point
#include "../utils/colors.h"   // For COLOR
// #include <GL/gl.h>          // gl.h is included by glew.h

// Structure to hold raw geometry data parsed from an OBJ file
typedef struct {
    char *name;
    Point *vertices;    // Array of vertex positions
    COLOR *colors;      // Array of vertex colors (1 per vertex)
    GLuint *indices;    // Array of indices for indexed drawing
    int num_vertices;
    int num_colors;     // Should match num_vertices
    int num_indices;
    // Bounding box or other metadata could be added here if needed
} OBJ_Data;


// Reads an OBJ file and populates the OBJ_Data structure.
// Returns NULL on failure. The caller is responsible for freeing
// the allocated memory within OBJ_Data (vertices, colors, indices, name)
// and OBJ_Data itself.
OBJ_Data *read_obj_file_data(const char *filename);



#endif //OBJ_PARSER_H
