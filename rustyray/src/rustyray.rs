use itertools_num::linspace;
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_PI_2, PI};
use std::ops;
use std::ops::{Add, AddAssign, Mul};

use ndarray::{arr1, arr2, arr3, Array1, Array2};

pub type Vec3f = Array1<f64>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3f,
    pub dir: Vec3f,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub origin: Vec3f,
    pub dir: Vec3f,
}
impl Camera {
    pub fn rotate(&mut self, mat: &Array2<f64>) {
        self.dir = mat.dot(&self.dir);

        let rmat = rotation_matrix(&arr1(&[0.0, 1.0, 0.0]), FRAC_PI_2);
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> f64 {
        match self {
            Object::Sphere(x) => x.intersect(ray),
        }
    }
    pub fn normal(&self, v: &Vec3f) -> Vec3f {
        match self {
            Object::Sphere(x) => x.get_normal(v),
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Object::Sphere(x) => &x.material.color,
        }
    }

    pub fn diffuse_c(&self) -> f64 {
        match self {
            Object::Sphere(x) => x.material.diffuse_c,
        }
    }

    pub fn specular_c(&self) -> f64 {
        match self {
            Object::Sphere(x) => x.material.specular_c,
        }
    }

    pub fn specular_k(&self) -> f64 {
        match self {
            Object::Sphere(x) => x.material.specular_k,
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Object::Sphere(Sphere { material, .. }) => material,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Light {
    pub pos: Vec3f,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn trace_ray(&self, ray: &Ray) -> Option<(usize, Vec3f, Vec3f, Color)> {
        let mut mindist = f64::INFINITY;
        let mut minobj = 0;

        for (objidx, obj) in self.objects.iter().enumerate() {
            let dist = obj.intersect(ray);
            if dist < mindist {
                mindist = dist;
                minobj = objidx;
            }
        }
        if mindist == f64::INFINITY {
            return None;
        }
        let obj = &self.objects[minobj];
        let M = &ray.origin + (&ray.dir * mindist);
        let N = obj.normal(&M);

        let color = obj.color();
        let to0 = ray.origin.clone() - &M;
        let to0: Vec3f = (&to0) / norm_vec(&to0);

        let mut col_ray = *color * 0.1f64;

        for (lidx, light) in self.lights.iter().enumerate() {
            let n = norm_vec(&(light.pos.clone() - &M));
            let toL: Vec3f = light.pos.clone() / n;

            let l = self
                .objects
                .iter()
                .enumerate()
                .filter_map(|(idx, o)| {
                    if idx != minobj {
                        Some(o.intersect(&Ray {
                            origin: M.clone() + N.clone() * 0.0001,
                            dir: toL.clone(),
                        }))
                    } else {
                        None
                    }
                })
                .reduce(f64::min)
                .map(|x| x < f64::INFINITY)
                .unwrap_or_default();
            if !l {
                col_ray += *color * (N.dot(&toL).max(0.0) * obj.diffuse_c());

                let lolnomrm = normalize(&(toL + &to0));
                col_ray += light.color
                    * (obj.specular_c() * N.dot(&lolnomrm).max(0.0).powf(obj.specular_k()))
            }
            //      if l and min(l) < np.inf:
        }

        Some((minobj, M, N, col_ray))
    }
    pub fn raycalc(&self, ray: &Ray, depth_max: u64) -> Color {
        let mut col = BLACK;
        let mut reflection = 1.0;

        for depth in 0..depth_max {
            if let Some((minobj, M, N, col_ray)) = self.trace_ray(&ray) {
                let rayO = &M + &N * 0.0001;
                let lol = &N * 2.0 * ray.dir.dot(&N);
                let rayD = normalize(&(ray.dir.clone() - lol));

                col += col_ray * reflection;
                reflection *= self.objects[minobj].material().reflection;
            } else {
                break;
            }
        }
        col
        //   def raycalc(scene, rayO,rayD,depth_max=3):
        //       reflection = 1.
        //       col = np.zeros(3)
        //       depth = 0
        //       # Loop through initial and secondary rays.
        //       while depth < depth_max:
        //           traced = trace_ray(scene,rayO, rayD )
        //           if not traced:
        //               break
        //           obj, M, N, col_ray = traced
        //           # Reflection: create a new ray.
        //           rayO, rayD = M + N * .0001, normalize(rayD - 2 * np.dot(rayD, N) * N)
        //           depth += 1
        //           col += reflection * col_ray
        //           reflection *= obj.get('reflection', 1.)
        //
        //       return col
    }

    pub fn enumerate_rays(self) {}
    // for for  .. w,h
    pub fn render(&self, w: usize, h: usize) {
        let rmat = rotation_matrix(&arr1(&[0.0, 1.0, 0.0]), FRAC_PI_2);
        let planeLoc = &self.camera.origin + &self.camera.dir;
        let orthx = rmat.dot(&self.camera.dir);

        let orthy = rotation_matrix(&self.camera.dir, FRAC_PI_2).dot(&orthx);
        let r = (w as f64) / (h as f64);

        //S = (-1., -1. / r + 0.25, 1., 1. / r + 0.25)
        let mut imgbuf = image::RgbImage::new(w as u32, h as u32);

        for (i, x) in linspace::<f64>(-1., 1., w).enumerate() {
            for (j, y) in linspace::<f64>(-1. / r + 0.25, 1. / r + 0.25, h).enumerate() {
                let mut col = BLACK;
                let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                let rayD = normalize(&((&planeLoc + lol) - &self.camera.origin));
                let ray = Ray {
                    origin: self.camera.origin.clone(),
                    dir: rayD,
                };
                col = self.raycalc(&ray, 3);

                imgbuf[(i as u32, j as u32)] = col.toRGB();
            }
        }

        imgbuf.save("lol.png");
    }
    // O,Q = scene.camera
    //
    // rmat = rotation_matrix([0,1,0], np.pi/2.)
    // camDir = Q#normalize(Q - O)
    //
    // planeLoc = O+camDir
    // orthx = rmat.dot(camDir)
    //
    // orthy = rotation_matrix(camDir, np.pi / 2.).dot(orthx)
    // log.info("camDir: %s  orth: %s", camDir, orthx)
    //
    // r = float(w) / h
    // # Screen coordinates: x0, y0, x1, y1.
    // S = (-1., -1. / r + .25, 1., 1. / r + .25)
    //
    // img = np.zeros((h, w, 3))
    //
    // col = np.zeros(3)  # Current color.
    // # Loop through all pixels.
    // for i, x in enumerate(np.linspace(S[0], S[2], w)):
    //     if i % 10 == 0:
    //         print(i / float(w) * 100, "%")
    //     for j, y in enumerate(np.linspace(S[1], S[3], h)):
    //         col[:] = 0
    //
    //         rayD = normalize((planeLoc + [x*orthx[0] , orthy[1]*y ,x*orthx[2] ]) - O)
    //         rayO = O
    //
    //         col = raycalc(scene, rayO, rayD, depth_max=depth_max )
    //
    //         img[h - j - 1, i, :] = np.clip(col, 0, 1)
    //
    // if figname is None:
    //     return img
    // else:
    //     plt.imsave(figname, img)
    // }
}

pub const BLACK: Color = Color([0.0, 0.0, 0.0]);
pub const RED: Color = Color([1.0, 0.0, 0.0]);
pub const BLUE: Color = Color([0.0, 0.0, 1.0]);
pub const WHITE: Color = Color([1.0, 1.0, 1.0]);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color([f64; 3]);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color([r, g, b])
    }
    fn norm(&self) -> f64 {
        return 1.0;
    }
    fn toRGB(&self) -> image::Rgb<u8> {
        let r8 = (self.0[0] * 255.0) as u8;
        let g8 = (self.0[1] * 255.0) as u8;
        let b8 = (self.0[2] * 255.0) as u8;

        image::Rgb([r8, g8, b8])
    }
}

impl From<[f64; 3]> for Color {
    fn from(x: [f64; 3]) -> Color {
        Color(x)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, right: Color) -> Color {
        Color([
            self.0[0] + right.0[0],
            self.0[1] + right.0[1],
            self.0[2] + right.0[2],
        ])
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, right: Color) {
        self.0[0] += right.0[0];
        self.0[1] += right.0[1];
        self.0[2] += right.0[2];
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

//impl Add<f64> for Color { ... }

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub reflection: f64,
    pub diffuse_c: f64,
    pub specular_c: f64,
    pub specular_k: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Vec3f,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> f64 {
        return intersect_sphere(&ray.origin, &ray.dir, self);
    }
    pub fn get_normal(&self, loc: &Vec3f) -> Vec3f {
        let n = loc - &self.origin;
        let norm = norm_vec(&n);
        //normalize_inplace(&mut n);
        return n / norm;
    }
    fn get_normal_ip(&self, loc: &Vec3f, out: &mut Vec3f) {
        *out = self.get_normal(loc)
    }
}

pub fn rotation_matrix(axis: &Vec3f, theta: f64) -> Array2<f64> {
    let a = (theta / 2.0).cos();

    let normaxis = norm_vec(axis);
    let axis_norm = axis / normaxis;

    let sin = (theta / 2.0).sin();

    let baaa = -axis_norm * sin;

    let b = baaa[0];
    let c = baaa[1];
    let d = baaa[2];

    // b, c, d = -axisx * sin;
    let aa = a * a;
    let bb = b * b;
    let cc = c * c;
    let dd = d * d;

    let bc = b * c;
    let ad = a * d;
    let ac = a * c;
    let ab = a * b;
    let bd = b * d;
    let cd = c * d;

    arr2(&[
        [aa + bb - cc - dd, 2. * (bc + ad), 2. * (bd - ac)],
        [2. * (bc - ad), aa + cc - bb - dd, 2. * (cd + ab)],
        [2. * (bd + ac), 2. * (cd - ab), aa + dd - bb - cc],
    ])
}

pub fn normalize(v: &Vec3f) -> Vec3f {
    let n = norm_vec(v);
    v / n
}

//pub fn normalize_inplace(v:&mut Vec3f) -> &Vec3f{
//    let n =  norm_vec(v);
//    *v = *v/n;
//    return v
//}

pub fn norm_vec_2(v: &Vec3f) -> f64 {
    return v.dot(v);
}

pub fn norm_vec(v: &Vec3f) -> f64 {
    norm_vec_2(v).sqrt()
}

pub fn intersect_sphere(loc: &Vec3f, dir: &Vec3f, s: &Sphere) -> f64 {
    // Return the distance from O to the intersection of the ray (O, D) with the
    // sphere (S, R), or +inf if there is no intersection.
    // O and S are 3D points, D (direction) is a normalized vector, R is a scalar.

    use std::cmp;

    let a = norm_vec_2(dir);
    let OS = loc - &s.origin;
    let b = 2.0 * dir.dot(&OS);
    let c = OS.dot(&OS) - &s.radius * &s.radius;

    let disc = b * b - 4. * a * c;
    if disc > 0.0 {
        let distSqrt = disc.sqrt();
        let mut q = 1.0;
        if b < 0. {
            q = (-b - distSqrt) / 2.0;
        } else {
            q = (-b + distSqrt) / 2.0;
        }

        let mut t0 = q / a;
        let mut t1 = c / q;

        t0 = t0.min(t1);
        t1 = t0.max(t1);
        if t1 >= 0. {
            if t0 < 0. {
                return t1;
            } else {
                return t0;
            }
        }
    }

    return f64::INFINITY;
}

pub fn raycalc(ray: Ray, depth: u64) -> Color {
    [0., 0., 3.].into()
}
