#version 100
precision highp float;
varying vec2 uv;
uniform float iTime;
uniform vec2 iResolution;

// Function to create a rectangular grid pattern
vec2 grid(vec2 p, float zoom) {
    p *= zoom;
    return fract(p);
}

// Function to create a smooth star shape
float star(vec2 p, float size) {
    float r = length(p);
    float a = atan(p.y, p.x);
    float f = smoothstep(size, size * 0.5, abs(r - (size * 0.8 + sin(a * 5.0 + iTime) * size * 0.15)));
    return f;
}

void main() {
    // Create a tiled grid
    vec2 p = gl_FragCoord.xy / iResolution.xy;
    p = grid(p, 20.0); // Adjust the 8.0 to change the number of tiles

    // Create a star pattern in each tile
    float s = star(p - 0.5, 0.5);

    // Base color with iridescence
    vec3 color = 0.5 + 0.5 * cos(iTime + uv.xyx * 8.0 + vec3(0, 2, 4));

    // Add color warping
    color += 0.2 * cos(12.0 * p.x + iTime) * cos(12.0 * p.y + iTime);

    // Apply the star pattern
    color *= s;

    // Add shimmering effect
    float shimmer = sin((uv.x + uv.y) * 40.0 - iTime * 3.0) * 0.5 + 0.5;
    color += shimmer * 0.1;

    // Enhance contrast
    color = pow(color, vec3(0.8));

    gl_FragColor = vec4(color, 1.0);
}
