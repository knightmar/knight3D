// Define feature test macro for strdup (POSIX 2008) before including any headers
#define _POSIX_C_SOURCE 200809L

#include "obj_parser.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define INITIAL_CAPACITY 256 // Initial capacity for dynamic arrays

// Helper to add a point to a dynamic array of points
static bool add_point_to_array(Point** array, int* count, int* capacity, Point p) {
    if (*count >= *capacity) {
        *capacity *= 2;
        Point* new_array = realloc(*array, *capacity * sizeof(Point));
        if (!new_array) {
            fprintf(stderr, "OBJ Parser: Failed to realloc points array\n");
            return false;
        }
        *array = new_array;
    }
    (*array)[(*count)++] = p;
    return true;
}

// Manual replacement for strdup for portability/debugging
static char* manual_strdup(const char* s) {
    if (!s) return NULL;
    size_t len = strlen(s) + 1;
    char* new_s = malloc(len);
    if (new_s) {
        memcpy(new_s, s, len);
    } else {
        fprintf(stderr, "manual_strdup: malloc failed\n");
    }
    return new_s;
}

// Helper to add a GLuint (index) to a dynamic array of GLuints
static bool add_index_to_array(GLuint** array, int* count, int* capacity, GLuint index) {
    if (*count >= *capacity) {
        *capacity *= 2;
        GLuint* new_array = realloc(*array, *capacity * sizeof(GLuint));
        if (!new_array) {
            fprintf(stderr, "OBJ Parser: Failed to realloc indices array\n");
            return false;
        }
        *array = new_array;
    }
    (*array)[(*count)++] = index;
    return true;
}

// Helper function to free all allocated OBJ_Data resources (currently unused)
/*
static void free_obj_data_internals(OBJ_Data *data) {
     if (data) {
        free(data->name);
        free(data->vertices);
        free(data->colors);
        free(data->indices);
    }
}
*/

OBJ_Data *read_obj_file_data(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (!file) {
        fprintf(stderr, "OBJ Parser: Cannot open file %s\n", filename);
        return NULL;
    }

    OBJ_Data *data = calloc(1, sizeof(OBJ_Data));
    if (!data) {
        fprintf(stderr, "OBJ Parser: Failed to allocate OBJ_Data\n");
        fclose(file);
        return NULL;
    }

    data->name = manual_strdup(filename); // Use manual strdup
    if (!data->name) {
        fprintf(stderr, "OBJ Parser: Failed to allocate name string\n");
        free(data);
        fclose(file);
        return NULL;
    }

    Point *temp_vertices = NULL; // Initialize to NULL
    GLuint *temp_indices = NULL; // Initialize to NULL

    temp_vertices = malloc(INITIAL_CAPACITY * sizeof(Point));
    int temp_v_count = 0;
    int temp_v_capacity = INITIAL_CAPACITY;

    temp_indices = malloc(INITIAL_CAPACITY * sizeof(GLuint));
    int temp_i_count = 0;
    int indices_capacity = INITIAL_CAPACITY;

    // Check initial allocations
    if (!temp_vertices || !temp_indices) {
        fprintf(stderr, "OBJ Parser: Failed initial allocation for temp arrays\n");
        free(temp_vertices); // Safe to free NULL
        free(temp_indices);  // Safe to free NULL
        free(data->name);
        free(data);
        fclose(file);
        return NULL;
    }

    char line[256];
    bool error_occurred = false;
    int line_num = 0;
    while (fgets(line, sizeof(line), file)) {
        line_num++;
        // printf("DEBUG Line %d: %s", line_num, line); // Keep this commented for now

        if (line[0] == 'v' && line[1] == ' ') { // Vertex position
            Point p;
            if (sscanf(line, "v %f %f %f", &p.x, &p.y, &p.z) == 3) {
                if (!add_point_to_array(&temp_vertices, &temp_v_count, &temp_v_capacity, p)) {
                    error_occurred = true; break;
                }
            }
        } else if (line[0] == 'f' && line[1] == ' ') { // Face
            // Use strtok and sscanf to handle "v/vt/vn" formats robustly
            char *line_ptr = line + 2; // Start after "f "
            char *token;
            int face_v_idx[4];
            int component_count = 0;

            while (component_count < 4 && (token = strtok(line_ptr, " \t\n")) != NULL) {
                line_ptr = NULL; // For subsequent strtok calls
                int parsed_idx;
                int sscanf_result = sscanf(token, "%d", &parsed_idx);
                if (sscanf_result == 1) {
                     face_v_idx[component_count] = parsed_idx - 1; // Convert to 0-based
                     component_count++;
                } else {
                    fprintf(stderr, "OBJ Parser: Warning - failed to parse integer from face component '%s'\n", token);
                    component_count = -1; // Mark as error
                    break;
                }
            }
            int items_read = component_count; // Use the count from strtok loop

            if (items_read < 3) {
               // This might happen if parsing failed or line had < 3 components
               if (items_read != -1) { // Don't print skip message if parse failed warning already printed
                 fprintf(stderr, "OBJ Parser: Skipping face - less than 3 valid vertex indices found (%d).\n", items_read);
               }
               continue;
            }

            // Validate indices
            bool indices_valid = true;
            for (int i = 0; i < items_read; ++i) {
                if (face_v_idx[i] < 0 || face_v_idx[i] >= temp_v_count) {
                    fprintf(stderr, "OBJ Parser: Error - Face index %d (value %d) is out of bounds (max vertex index %d).\n",
                            i, face_v_idx[i] + 1, temp_v_count);
                    indices_valid = false;
                    break;
                }
            }

            if (!indices_valid) {
                fprintf(stderr, "OBJ Parser: Skipping face due to invalid index.\n");
                continue;
            }

            // Add indices for the first triangle
            if (!add_index_to_array(&temp_indices, &temp_i_count, &indices_capacity, face_v_idx[0]) ||
                !add_index_to_array(&temp_indices, &temp_i_count, &indices_capacity, face_v_idx[1]) ||
                !add_index_to_array(&temp_indices, &temp_i_count, &indices_capacity, face_v_idx[2])) {
                error_occurred = true; break;
            }

            // Add indices for the second triangle if it was a quad
            if (items_read == 4) {
                if (!add_index_to_array(&temp_indices, &temp_i_count, &indices_capacity, face_v_idx[0]) ||
                    !add_index_to_array(&temp_indices, &temp_i_count, &indices_capacity, face_v_idx[2]) ||
                    !add_index_to_array(&temp_indices, &temp_i_count, &indices_capacity, face_v_idx[3])) {
                    error_occurred = true; break;
                }
            }
        } // End of "else if (f...)"
    } // End of "while(fgets...)"
    fclose(file);

    if (error_occurred) {
        free(temp_vertices);
        free(temp_indices);
        free(data->name);
        free(data);
        return NULL;
    }

    // Copy unique vertices
    data->vertices = malloc(temp_v_count * sizeof(Point));
    if (!data->vertices) {
        fprintf(stderr, "OBJ Parser: Failed to allocate final vertices array\n");
        free(temp_vertices); free(temp_indices); free(data->name); free(data); return NULL;
    }
    memcpy(data->vertices, temp_vertices, temp_v_count * sizeof(Point));
    data->num_vertices = temp_v_count;
    free(temp_vertices);
    temp_vertices = NULL;

    // Generate default colors
    data->colors = malloc(data->num_vertices * sizeof(COLOR));
    if (!data->colors) {
        fprintf(stderr, "OBJ Parser: Failed to allocate colors array\n");
        free(data->vertices); free(temp_indices); free(data->name); free(data); return NULL;
    }
    data->num_colors = data->num_vertices;
    for (int i = 0; i < data->num_vertices; ++i) {
        data->colors[i] = (COLOR){255, 255, 255}; // Default to white
    }

    // Assign indices
    data->indices = temp_indices;
    data->num_indices = temp_i_count;

    printf("DEBUG PARSER: Final temp_i_count = %d\n", temp_i_count); // Keep this debug print
    printf("OBJ Parser: Loaded %s - Vertices: %d, Indices: %d\n",
           data->name, data->num_vertices, data->num_indices);

    return data;
}
