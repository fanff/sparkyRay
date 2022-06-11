#version 330 core

layout (location = 0) in vec3 origin;
//layout (location = 1) in vec3 orthx  ;
//layout (location = 2) in vec3 orthy  ;
//layout (location = 3) in vec3 direction;

void main()
{
    gl_Position = vec4(origin, 1.0);
}