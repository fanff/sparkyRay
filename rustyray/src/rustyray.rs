use std::ops;

use ndarray::{arr1, arr2,arr3,Array1,Array2};

pub type Vec3f = Array1<f64>;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray<'a>{
    pub origin: &'a Vec3f,
    pub dir: &'a Vec3f
}





#[derive(Debug, Clone, PartialEq)]
pub struct Sphere{
    pub origin: Vec3f,
    pub radius: f64
}
impl Sphere {
    pub fn intersect(&self,ray : &Ray) -> f64 {
        return intersect_sphere(ray.origin,ray.dir,self);
    }
    pub fn get_normal(&self,loc: &Vec3f) -> &Vec3f{
        let n = loc - self.origin;
        let norm = norm_vec(&n);
        //normalize_inplace(&mut n);
        return &(n / norm)
    }
}

pub fn rotation_matrix(axis:&Vec3f,theta:f64) -> Array2<f64> {
    let a = (theta / 2.0).cos();

    let normaxis = norm_vec(axis);
    let axis_norm = axis / normaxis;

    let sin = (theta / 2.0).sin();

    let baaa = -axis_norm*sin;

    let b = baaa[0];
    let c = baaa[1];
    let d = baaa[2];

    // b, c, d = -axisx * sin;
    let aa= a * a;
    let bb= b * b;
    let cc= c * c;
    let dd= d * d;

    let bc= b * c;
    let ad= a *d;
    let ac= a *c;
    let ab= a *b;
    let bd= b *d;
    let cd= c *d;
    return arr2(&[[aa + bb - cc - dd, 2. * (bc + ad), 2. * (bd - ac)],
                     [2. * (bc - ad), aa + cc - bb - dd, 2. * (cd + ab)],
                     [2. * (bd + ac), 2. * (cd - ab), aa + dd - bb - cc]])
}

// pub fn normalize(v:&Vec3f) -> Vec3f{
//
//     let n =  norm_vec(v);
//     return Vec3f{x : &v.x/n,
//                 y : &v.y/n,
//                 z : &v.z/n}
// }


//pub fn normalize_inplace(v:&mut Vec3f) -> &Vec3f{
//    let n =  norm_vec(v);
//    *v = *v/n;
//    return v
//}




pub fn norm_vec_2(v : & Vec3f) -> f64 {
    return v.dot(v)
}

pub fn norm_vec(v:& Vec3f) -> f64 { norm_vec_2(v).sqrt()}



pub fn intersect_sphere(loc:& Vec3f,dir:& Vec3f, s:&Sphere)->f64{


    // Return the distance from O to the intersection of the ray (O, D) with the
    // sphere (S, R), or +inf if there is no intersection.
    // O and S are 3D points, D (direction) is a normalized vector, R is a scalar.

     use std::cmp;


    let a = norm_vec_2(dir);
    let OS = loc - &s.origin;
    let b = 2.0 * dir.dot(&OS);
    let c = OS.dot(&OS) - &s.radius*&s.radius;

    let disc = b * b - 4. * a * c;
    if disc > 0.0 {

        let distSqrt = disc.sqrt();
        let mut q = 1.0;
        if b < 0. {
            q = (-b - distSqrt) / 2.0;
        }else{
            q = (-b + distSqrt) / 2.0;
        }



        let mut t0 = q / a;
        let mut t1 = c / q;


        t0 = t0.min(t1);
        t1 = t0.max(t1);
        if t1 >= 0. {
            if t0<0. {
                return t1
            }
            else{
                return t0
            }
        }

    }

    return f64::INFINITY


}
