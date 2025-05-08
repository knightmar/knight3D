#include "shader.h"
#include <stdio.h>
#include <stdlib.h>

GLuint load_shaders(const char* vertex_file_path, const char* fragment_file_path) {
    // Create the shaders
    GLuint vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    GLuint fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);

    // Read the Vertex Shader code from file
    FILE* vertex_file = fopen(vertex_file_path, "rb");
    if (!vertex_file) {
        fprintf(stderr, "Error opening vertex shader file\n");
        return 0;
    }
    fseek(vertex_file, 0, SEEK_END);
    long vertex_file_size = ftell(vertex_file);
    rewind(vertex_file);
    char* vertex_shader_code = (char*)malloc(vertex_file_size + 1);
    fread(vertex_shader_code, 1, vertex_file_size, vertex_file);
    vertex_shader_code[vertex_file_size] = '\0';
    fclose(vertex_file);

    // Read the Fragment Shader code from file
    FILE* fragment_file = fopen(fragment_file_path, "rb");
    if (!fragment_file) {
        fprintf(stderr, "Error opening fragment shader file\n");
        free(vertex_shader_code);
        return 0;
    }
    fseek(fragment_file, 0, SEEK_END);
    long fragment_file_size = ftell(fragment_file);
    rewind(fragment_file);
    char* fragment_shader_code = (char*)malloc(fragment_file_size + 1);
    fread(fragment_shader_code, 1, fragment_file_size, fragment_file);
    fragment_shader_code[fragment_file_size] = '\0';
    fclose(fragment_file);

    // Compile Vertex Shader
    glShaderSource(vertex_shader, 1, (const char**)&vertex_shader_code, NULL);
    glCompileShader(vertex_shader);

    // Check Vertex Shader
    GLint success;
    GLchar info_log[512];
    glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vertex_shader, 512, NULL, info_log);
        fprintf(stderr, "Vertex shader compilation error: %s\n", info_log);
        free(vertex_shader_code);
        free(fragment_shader_code);
        return 0;
    }

    // Compile Fragment Shader
    glShaderSource(fragment_shader, 1, (const char**)&fragment_shader_code, NULL);
    glCompileShader(fragment_shader);

    // Check Fragment Shader
    glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(fragment_shader, 512, NULL, info_log);
        fprintf(stderr, "Fragment shader compilation error: %s\n", info_log);
        free(vertex_shader_code);
        free(fragment_shader_code);
        return 0;
    }

    // Link the program
    GLuint program = glCreateProgram();
    glAttachShader(program, vertex_shader);
    glAttachShader(program, fragment_shader);
    glLinkProgram(program);

    // Check the program
    glGetProgramiv(program, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(program, 512, NULL, info_log);
        fprintf(stderr, "Shader program linking error: %s\n", info_log);
        free(vertex_shader_code);
        free(fragment_shader_code);
        return 0;
    }

    // Clean up shaders
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);
    free(vertex_shader_code);
    free(fragment_shader_code);

    return program;
}
