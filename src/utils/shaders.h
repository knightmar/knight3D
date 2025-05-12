//
// Created by knightmar on 03/05/25.
//

#ifndef SHADERS_H
#define SHADERS_H
#include "../renderer.h"


GLuint shader_program;

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

void initialize_shaders();


#endif //SHADERS_H
