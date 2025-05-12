//
// Created by knightmar on 03/05/25.
//

#include "shaders.h"

const char *vertex_shader_source = R"(
    #version 130
    in vec3 position;
    in vec3 color;
    out vec3 frag_color;
    uniform mat4 modelview;
    uniform mat4 projection;

    void main() {
        frag_color = color;
        gl_Position = projection * modelview * vec4(position, 1.0);
    }
)";


const char *fragment_shader_source = R"(
    #version 130
    in vec3 frag_color;
    out vec4 frag_color_out;

    void main() {
        frag_color_out = vec4(frag_color, 1.0);
    }
)";

void initialize_shaders() {
    GLuint vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertex_shader, 1, &vertex_shader_source, NULL);
    glCompileShader(vertex_shader);

    GLuint fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragment_shader, 1, &fragment_shader_source, NULL);
    glCompileShader(fragment_shader);

    shader_program = glCreateProgram();
    glAttachShader(shader_program, vertex_shader);
    glAttachShader(shader_program, fragment_shader);
    glLinkProgram(shader_program);
    glUseProgram(shader_program);
}
