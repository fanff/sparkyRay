https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution



```c

bool rayTriangleIntersect( 
    const Vec3f &orig, const Vec3f &dir, 
    const Vec3f &v0, const Vec3f &v1, const Vec3f &v2, 
    float &t, float &u, float &v) 
{ 
    Vec3f v0v1 = v1 - v0; 
    Vec3f v0v2 = v2 - v0; 
    Vec3f pvec = dir.crossProduct(v0v2); 
    float det = v0v1.dotProduct(pvec); 
#ifdef CULLING 
    // if the determinant is negative the triangle is backfacing
    // if the determinant is close to 0, the ray misses the triangle
    if (det < kEpsilon) return false; 
#else 
    // ray and triangle are parallel if det is close to 0
    if (fabs(det) < kEpsilon) return false; 
#endif 
    float invDet = 1 / det; 
 
    Vec3f tvec = orig - v0; 
    u = tvec.dotProduct(pvec) * invDet; 
    if (u < 0 || u > 1) return false; 
 
    Vec3f qvec = tvec.crossProduct(v0v1); 
    v = dir.dotProduct(qvec) * invDet; 
    if (v < 0 || u + v > 1) return false; 
 
    t = v0v2.dotProduct(qvec) * invDet; 
 
    return true; 
```