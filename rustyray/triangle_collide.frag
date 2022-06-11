#version 330 core

#define FLT_MAX 3.402823466e+38
#define FLT_MIN 1.175494351e-38
#define DBL_MAX 1.7976931348623158e+308
#define DBL_MIN 2.2250738585072014e-308

#define TCOUNT 1024
#define LIGHT_COUNT 16

// The name of the block is used for finding the index location only
layout (std140) uniform myArrayBlock {
  vec3  myArray [TCOUNT*3]; // This is the important name (in the shader).
};

layout (std140) uniform lightBlock {
  vec3  light_pos[LIGHT_COUNT]; // This is the important name (in the shader).
  vec3  light_color[LIGHT_COUNT];
};

// camera  settings
layout (std140) uniform cameraBlock {
    float cameraBlockArray[12];
};




vec4 rayTriangleIntersect(
    vec3 orig, vec3 dir,
    vec3 v0 ,
    vec3 v1 ,
    vec3 v2
    );
vec4 filter_inter(vec4 intersectInfo ,vec4 res );


out vec4 projection_result;

void  main(){

    vec3 cam_origin = vec3 (cameraBlockArray[0],cameraBlockArray[1],cameraBlockArray[2]);
    vec3 orthx  = vec3 (cameraBlockArray[3],cameraBlockArray[4],cameraBlockArray[5]);
    vec3 orthy  = vec3 (cameraBlockArray[6],cameraBlockArray[7],cameraBlockArray[8]);
    vec3 cam_direction= vec3 (cameraBlockArray[9],cameraBlockArray[10],cameraBlockArray[11]);


    float x = gl_FragCoord.x/300f; // viewport in 0/1
    float y = gl_FragCoord.y/300f; // viewport in 0/1

    vec3 rayDir = cam_direction + (orthx * x )+ (orthy * y);

    // u,v, whatever , distance
    vec4 nearest_object = vec4(0,0,0,FLT_MAX);


    // ray to triangles;
    #pragma unroll TCOUNT
    for(int  i=0;i<TCOUNT;i++){
            vec4 intersectInfo =rayTriangleIntersect(cam_origin,rayDir,  myArray[0], myArray[1], myArray[2]);
            nearest_object = filter_inter( intersectInfo , nearest_object );

    }

    // ray to light;
    vec4 lightInter = vec4(0,0,0,FLT_MAX);
    #pragma unroll LIGHT_COUNT
    for(int lightidx=0;lightidx<LIGHT_COUNT;lightidx++){

        #pragma unroll TCOUNT
        for(int  i=0;i<TCOUNT;i++){
                vec3 lightRay = cam_origin + (rayDir*nearest_object.z) - light_pos[lightidx];

                vec4 lightrayInter =rayTriangleIntersect( light_pos[lightidx]  ,normalize(lightRay), myArray[0], myArray[1], myArray[2]);
                lightInter = filter_inter(lightrayInter,lightInter);
        }
    }

    projection_result = nearest_object;

    if (x<.5){
        if (y>.5){

            projection_result = vec4(cam_origin,1);
        }else{
            projection_result = vec4(orthx,1);
        }

    }else{
        if (y>.5){
            projection_result = vec4(orthy,1);
        }else{
            projection_result = vec4(cam_direction,1);
        }
    }

}


vec4 filter_inter(vec4 intersectInfo ,vec4 res ) {
    float u = intersectInfo.x;
    float v = intersectInfo.y;
    float det = intersectInfo.z;
    float t = intersectInfo.w;

    if (! ((det < 0.00001) || (v < 0 || u + v > 1)  || (u < 0 || u > 1)) ){
        if (t< res.z){
            return intersectInfo;
        }
    }
    return res;

}


vec4 rayTriangleIntersect(
    vec3 orig, vec3 dir,
    vec3 v0 ,
    vec3 v1 ,
    vec3 v2
    )
{

    float t = 0;
    float u = 0;
    float v = 0;

    vec3 v0v1 = v1 - v0;
    vec3 v0v2 = v2 - v0;
    vec3 pvec = cross(dir,v0v2);
    float det = dot(v0v1,pvec);

    // if the determinant is negative the triangle is backfacing
    // if the determinant is close to 0, the ray misses the triangle
    //if (det < kEpsilon) return false;

    float invDet = 1 / det;

    vec3 tvec = orig - v0;
    u = dot(tvec,pvec) * invDet;
    //if (u < 0 || u > 1) return false;

    vec3 qvec = cross(tvec,v0v1);
    v = dot(dir,qvec) * invDet;
    //if (v < 0 || u + v > 1) return false;

    t = dot(v0v2,qvec) * invDet;

    return vec4 (u,v,t,det);
}