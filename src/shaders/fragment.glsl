#version 330 core

in vec3 fragmentColor; // Color from vertex attribute
out vec4 finalColor;   // Output color

uniform bool useOverrideColor; // Flag to use override color
uniform vec3 overrideColor;    // Color to use for override (e.g., edges)

void main() {
    if (useOverrideColor) {
        finalColor = vec4(overrideColor, 1.0);
    } else {
        finalColor = vec4(fragmentColor, 1.0);
    }
}
