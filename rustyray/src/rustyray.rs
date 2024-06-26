pub mod game;
mod sdl_loader;

use itertools_num::linspace;
use serde::{Deserialize, Serialize};
use std::f64::consts::FRAC_PI_2;
use std::fs::File;
use std::io::BufReader;

use std::ops::{Add, AddAssign, Mul};

use ndarray::{arr1, arr2, Array1, Array2};
use sdl2::rect::Rect;
use sdl2::render::Texture;

pub type Vec3f = Array1<f64>;

use crate::sdl_loader::{load_stl, load_stl_debug};
use nalgebra::Vector3;

//pub type Vec3f = Vector3<f64>;

pub fn vec3f_zero() -> Vec3f {
    //Vector3([0., 0., 0.])
    arr1(&[0., 0., 0.])
}
pub fn vec3f_x() -> Vec3f {
    arr1(&[1., 0., 0.])
}
pub fn vec3f_y() -> Vec3f {
    arr1(&[0., 1.0, 0.])
}
pub fn vec3f_z() -> Vec3f {
    arr1(&[0., 0., 1.])
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3f,
    pub dir: Vec3f,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Camera {
    pub origin: Vec3f,
    pub dir: Vec3f,
    pub orthx: Vec3f,
    pub orthy: Vec3f,
}
impl Camera {
    pub fn move_speed(&mut self, speed: f64) {
        self.origin = &self.origin + &self.dir * speed;
    }
    pub fn move_side(&mut self, speed: f64) {
        self.origin = &self.origin + &self.orthx * speed;
    }
    pub fn rot_angl(&mut self, theta: f64) {
        let up = arr1(&[0.0, 1.0, 0.0]);
        let r = rotation_matrix(&up, theta);
        self.dir = r.dot(&self.dir);
        self.orthx = r.dot(&self.orthx);

        self.orthy = rotation_matrix(&self.dir, FRAC_PI_2).dot(&self.orthx);
    }
    pub fn rot_ud(&mut self, theta: f64) {
        self.dir[1] += theta;
        self.dir = normalize(&self.dir)
    }

    pub fn default() -> Camera {
        Camera {
            origin: vec3f_zero(),
            dir: vec3f_x(),
            orthx: vec3f_z(),
            orthy: vec3f_z(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Disc(Disc),
    Triangle(Triangle),
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> f64 {
        match self {
            Object::Sphere(x) => x.intersect(ray),
            Object::Plane(x) => x.intersect(ray),
            Object::Disc(x) => x.intersect(ray),
            Object::Triangle(x) => x.intersect(ray),
        }
    }
    pub fn normal(&self, v: &Vec3f) -> Vec3f {
        match self {
            Object::Sphere(x) => x.get_normal(v),
            Object::Plane(x) => x.get_normal(v),
            Object::Disc(x) => x.get_normal(v),
            Object::Triangle(x) => x.normal.clone(),
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Object::Sphere(Sphere { material, .. }) => material,
            Object::Plane(x) => &x.material,
            Object::Disc(x) => &x.material,
            Object::Triangle(x) => &x.material,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewZone {
    pub x_min: f64,
    pub x_len: f64,
    pub y_min: f64,
    pub y_len: f64,
    pub border: bool,
}
impl ViewZone {
    pub fn fullratio() -> ViewZone {
        ViewZone {
            x_min: -1.0,
            x_len: 2.0,
            y_min: -1.0,
            y_len: 2.0,
            border: false,
        }
    }
    pub fn split_border_n_ratio(&self, n_split: u8, tw: u32, th: u32) -> Vec<ViewZone> {
        if n_split == 1 {
            let mut m = self.clone();
            m.border = false;
            return [m].to_vec();
        }

        let x_len = self.x_len / tw as f64;
        let y_len = self.y_len / th as f64;

        let inner = ViewZone {
            x_min: self.x_min + x_len,
            x_len: x_len * (tw as f64 - 2.0),
            y_min: self.y_min + y_len,
            y_len: y_len * (th as f64 - 2.0),
            border: true,
        };
        let subs = inner.split_border_n_ratio(n_split - 1, tw, th);

        let mut m = self.clone();
        m.border = true;
        let mut me = [m].to_vec();

        //[self.clone()].concat([self.clone()]);

        return [me, subs].concat();
    }

    pub fn split_n_ratio(&self, x_split: u8, y_split: u8) -> Vec<ViewZone> {
        let x_len = self.x_len / x_split as f64;
        let y_len = self.y_len / y_split as f64;

        let lol = (0..x_split)
            .flat_map(|i| {
                (0..y_split).map(move |j| ViewZone {
                    x_min: self.x_min + x_len * i as f64,
                    x_len: x_len,
                    y_min: self.y_min + y_len * j as f64,
                    y_len: y_len,
                    border: false,
                })
            })
            .collect::<Vec<ViewZone>>();
        return lol;
    }

    pub fn to_sceen_rect(&self, w: u32, h: u32) -> Rect {
        let xi = (self.x_min + 1.0) / 2.0;
        let yi = (self.y_min + 1.0) / 2.0;

        let width = self.x_len / 2.0;
        let height = self.y_len / 2.0;
        //let r = w as f64 / h as f64;
        Rect::new(
            (xi * w as f64) as i32,
            (yi * h as f64) as i32,
            (w as f64 * width) as u32,
            (h as f64 * height) as u32,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Light {
    pub pos: Vec3f,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        let collision_point = &ray.origin + (&ray.dir * mindist);
        let collision_normal = obj.normal(&collision_point);

        let color = obj.material().color;
        let to0 = normalize(&(ray.origin.clone() - &collision_point));

        let mut col_ray = color * 0.1f64;

        for (lidx, light) in self.lights.iter().enumerate() {
            let to_light = normalize(&(light.pos.clone() - &collision_point));

            let l = self
                .objects
                .iter()
                .enumerate()
                .filter_map(|(idx, o)| {
                    if idx != minobj {
                        Some(o.intersect(&Ray {
                            origin: collision_point.clone() + collision_normal.clone() * 0.0001,
                            dir: to_light.clone(),
                        }))
                    } else {
                        None
                    }
                })
                .reduce(f64::min)
                .map(|dist_to_light| dist_to_light < f64::INFINITY)
                .unwrap_or_default();
            if !l {
                //  Lambert shading (diffuse).
                col_ray +=
                    color * (collision_normal.dot(&to_light).max(0.0) * obj.material().diffuse_c);

                // Blinn-Phong shading (specular).
                // let lolnomrm = normalize(&(&toL + &to0));
                col_ray += light.color
                    * (obj.material().specular_c
                        * collision_normal
                            .dot(&normalize(&(&to_light + &to0)))
                            .max(0.0)
                            .powf(obj.material().specular_k))
            }
            //      if l and min(l) < np.inf:
        }

        Some((minobj, collision_point, collision_normal, col_ray))
    }
    pub fn raycalc(&self, ray: &Ray, depth_max: u64) -> Color {
        let mut col = BLACK;
        let mut reflection = 1.0;
        let mut used_ray = ray.clone();

        for _depth in 0..depth_max {
            if let Some((minobj, collision_point, collision_normal, col_ray)) =
                self.trace_ray(&used_ray)
            {
                let ray_origin = &collision_point + (&collision_normal * 0.0001);
                let lol = &collision_normal * 2.0 * ray.dir.dot(&collision_normal);
                let ray_direction = normalize(&(used_ray.dir.clone() - lol));

                col += col_ray * reflection;
                reflection *= self.objects[minobj].material().reflection;

                used_ray.origin = ray_origin;
                used_ray.dir = ray_direction;
            } else {
                break;
            }
        }
        col
    }
    pub fn render_zone_to_texture(
        &self,
        w: usize,
        h: usize,
        view_zone: &ViewZone,
        depth: u64,
        texture: &mut Texture,
    ) {
        let rmat = rotation_matrix(&arr1(&[0.0, 1.0, 0.0]), FRAC_PI_2);
        let camera_plane_loc = &self.camera.origin + &self.camera.dir;
        let mut orthx = rmat.dot(&self.camera.dir);
        orthx[1] = 0.0;

        let orthy = rotation_matrix(&self.camera.dir, FRAC_PI_2).dot(&orthx);

        let r = (w as f64) / (h as f64);
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                //buffer.copy_from_slice(array);

                for i in 0..w {
                    for j in 0..h {
                        //
                        let ir = i as f64 / w as f64; // 0,1
                        let x = view_zone.x_min + (view_zone.x_len) * ir;
                        //let x = 2.0 * i as f64 / w as f64 - 1.0;

                        let y = view_zone.y_min + (view_zone.y_len) * ((j as f64) / h as f64);
                        //let y = (2.0 * j as f64 / r) / h as f64 - 1.0;
                        //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                        //let lol = &orthx * x + &orthy * y;
                        let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                        let rayD = normalize(&((&camera_plane_loc + lol) - &self.camera.origin));
                        let ray = Ray {
                            origin: self.camera.origin.clone(),
                            dir: rayD,
                        };
                        let col = self.raycalc(&ray, depth).to_rbg_tuple();

                        let offset = j * pitch + i * 3;
                        buffer[offset..offset + 3].copy_from_slice(&col);
                        //buffer[offset] = col[0];
                        //buffer[offset + 1] = col[1];
                        //buffer[offset + 2] = col[2];
                    }
                }
            })
            .unwrap();
        // flkdj
    }

    pub fn render_zone_to_buff(
        &self,
        w: usize,
        h: usize,
        view_zone: &ViewZone,
        depth: u64,
        buffer: &mut [u8],
    ) {
        if view_zone.border {
            return self.render_border_zone_to_buff(w, h, view_zone, depth, buffer);
        }
        let camera_plane_loc = &self.camera.origin + &self.camera.dir;
        let orthx = &self.camera.orthx;
        let orthy = &self.camera.orthy;

        let r = (w as f64) / (h as f64);

        for i in 0..w {
            for j in 0..h {
                //
                let ir = i as f64 / w as f64; // 0,1
                let x = view_zone.x_min + (view_zone.x_len) * ir;
                //let x = 2.0 * i as f64 / w as f64 - 1.0;

                let y = view_zone.y_min + (view_zone.y_len) * ((j as f64) / h as f64);
                //let y = (2.0 * j as f64 / r) / h as f64 - 1.0;
                //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);

                let x = x / 2.0;
                let y = y / 2.0;
                let lol = (orthx * x) + (orthy * y);
                //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                let rayD = normalize(&((&camera_plane_loc + lol) - &self.camera.origin));
                let ray = Ray {
                    origin: self.camera.origin.clone(),
                    dir: rayD,
                };
                let col = self.raycalc(&ray, depth).to_rbg_tuple();

                let offset = (j * w + i) * 3;

                buffer[offset..offset + 3].copy_from_slice(&col);
                // buffer[offset + 0] = col[0];
                // buffer[offset + 1] = col[1];
                // buffer[offset + 2] = col[2];
            }
        }
    }

    pub fn render_border_zone_to_buff(
        &self,
        w: usize,
        h: usize,
        view_zone: &ViewZone,
        depth: u64,
        buffer: &mut [u8],
    ) {
        let camera_plane_loc = &self.camera.origin + &self.camera.dir;
        let orthx = &self.camera.orthx;
        let orthy = &self.camera.orthy;

        let r = (w as f64) / (h as f64);

        for i in 0..w {
            for j in [0, h - 1] {
                //
                let ir = i as f64 / w as f64; // 0,1
                let x = view_zone.x_min + (view_zone.x_len) * ir;
                //let x = 2.0 * i as f64 / w as f64 - 1.0;

                let y = view_zone.y_min + (view_zone.y_len) * ((j as f64) / h as f64);
                //let y = (2.0 * j as f64 / r) / h as f64 - 1.0;
                //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);

                let x = x / 2.0;
                let y = y / 2.0;
                let lol = (orthx * x) + (orthy * y);
                //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                let rayD = normalize(&((&camera_plane_loc + lol) - &self.camera.origin));
                let ray = Ray {
                    origin: self.camera.origin.clone(),
                    dir: rayD,
                };
                let col = self.raycalc(&ray, depth).to_rbg_tuple();

                let offset = (j * w + i) * 3;

                buffer[offset..offset + 3].copy_from_slice(&col);
                // buffer[offset + 0] = col[0];
                // buffer[offset + 1] = col[1];
                // buffer[offset + 2] = col[2];
            }
        }

        for i in [0, w - 1] {
            for j in 1..h - 1 {
                //
                let ir = i as f64 / w as f64; // 0,1
                let x = view_zone.x_min + (view_zone.x_len) * ir;
                //let x = 2.0 * i as f64 / w as f64 - 1.0;

                let y = view_zone.y_min + (view_zone.y_len) * ((j as f64) / h as f64);
                //let y = (2.0 * j as f64 / r) / h as f64 - 1.0;
                //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);

                let x = x / 2.0;
                let y = y / 2.0;
                let lol = (orthx * x) + (orthy * y);
                //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                let rayD = normalize(&((&camera_plane_loc + lol) - &self.camera.origin));
                let ray = Ray {
                    origin: self.camera.origin.clone(),
                    dir: rayD,
                };
                let col = self.raycalc(&ray, depth).to_rbg_tuple();

                let offset = (j * w + i) * 3;

                buffer[offset..offset + 3].copy_from_slice(&col);
                // buffer[offset + 0] = col[0];
                // buffer[offset + 1] = col[1];
                // buffer[offset + 2] = col[2];
            }
        }

        //
    }

    pub fn render_to_texture(&self, w: usize, h: usize, depth: u64, texture: &mut Texture) {
        let rmat = rotation_matrix(&arr1(&[0.0, 1.0, 0.0]), FRAC_PI_2);
        let camera_plane_loc = &self.camera.origin + &self.camera.dir;
        let mut orthx = rmat.dot(&self.camera.dir);
        orthx[1] = 0.0;

        let orthy = rotation_matrix(&self.camera.dir, FRAC_PI_2).dot(&orthx);

        let r = (w as f64) / (h as f64);
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                //buffer.copy_from_slice(array);

                for i in 0..w {
                    for j in 0..h {
                        //
                        let x = 2.0 * i as f64 / w as f64 - 1.0;
                        let y = (2.0 * j as f64 / r) / h as f64 - 1.0;
                        //let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                        //let lol = &orthx * x + &orthy * y;
                        let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                        let rayD = normalize(&((&camera_plane_loc + lol) - &self.camera.origin));
                        let ray = Ray {
                            origin: self.camera.origin.clone(),
                            dir: rayD,
                        };
                        let col = self.raycalc(&ray, depth).to_rbg_tuple();

                        let offset = j * pitch + i * 3;
                        buffer[offset] = col[0];
                        buffer[offset + 1] = col[1];
                        buffer[offset + 2] = col[2];
                    }
                }
            })
            .unwrap();
        // flkdj
    }
    pub fn render(
        &self,
        w: usize,
        h: usize,
        depth: u64,
        limitxdown: f64,
        limitxup: f64,
        limitydown: f64,
        limityup: f64,
    ) -> image::RgbImage {
        let rmat = rotation_matrix(&arr1(&[0.0, 1.0, 0.0]), FRAC_PI_2);
        let camera_plane_loc = &self.camera.origin + &self.camera.dir;
        let orthx = rmat.dot(&self.camera.dir);

        let orthy = rotation_matrix(&self.camera.dir, FRAC_PI_2).dot(&orthx);
        let r = (w as f64) / (h as f64);

        let wspace = linspace::<f64>(limitxdown, limitxup, w);

        let mut imgbuf = image::RgbImage::new(w as u32, h as u32);

        for (i, x) in wspace.enumerate() {
            let hspace = linspace::<f64>(limitydown / r, limityup / r, h);

            for (j, y) in hspace.enumerate() {
                let lol = arr1(&[x * orthx[0], orthy[1] * y, x * orthx[2]]);
                let rayD = normalize(&((&camera_plane_loc + lol) - &self.camera.origin));
                let ray = Ray {
                    origin: self.camera.origin.clone(),
                    dir: rayD,
                };
                let col = self.raycalc(&ray, depth);

                imgbuf[(i as u32, (j) as u32)] = col.to_rbg();
            }
        }
        return imgbuf;
    }
}

pub const BLACK: Color = Color([0.0, 0.0, 0.0]);
pub const RED: Color = Color([1.0, 0.0, 0.0]);
pub const GREEN: Color = Color([0.0, 1.0, 0.0]);
pub const BLUE: Color = Color([0.0, 0.0, 1.0]);
pub const WHITE: Color = Color([1.0, 1.0, 1.0]);

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Color([f64; 3]);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color([r, g, b])
    }

    fn to_rbg(&self) -> image::Rgb<u8> {
        let r8 = (self.0[0] * 255.0) as u8;
        let g8 = (self.0[1] * 255.0) as u8;
        let b8 = (self.0[2] * 255.0) as u8;

        image::Rgb([r8, g8, b8])
    }

    fn to_rbg_tuple(&self) -> [u8; 3] {
        let r8 = (self.0[0] * 255.0) as u8;
        let g8 = (self.0[1] * 255.0) as u8;
        let b8 = (self.0[2] * 255.0) as u8;

        [r8, g8, b8]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    pub color: Color,
    pub reflection: f64,
    pub diffuse_c: f64,
    pub specular_c: f64,
    pub specular_k: f64,
}
impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::default(),
            reflection: 0.0,
            diffuse_c: 0.0,
            specular_c: 0.0,
            specular_k: 0.0,
        }
    }
    pub fn mat_color(c: Color) -> Material {
        Material {
            color: c,
            reflection: 0.0,
            diffuse_c: 0.0,
            specular_c: 0.0,
            specular_k: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub v0: Vec3f,
    pub v1: Vec3f,
    pub v2: Vec3f,
    pub normal: Vec3f,
    pub material: Material,
}
impl Triangle {
    pub fn new(v0: Vec3f, v1: Vec3f, v2: Vec3f, material: Material) -> Triangle {
        let mut t = Triangle {
            v0,
            v1,
            v2,
            normal: vec3f_zero(),
            material,
        };
        t.normal = t.get_normal();
        t
    }

    pub fn default() -> Triangle {
        let mut t = Triangle {
            v0: vec3f_zero(),
            v1: vec3f_x(),
            v2: vec3f_z() * 2.0,
            normal: vec3f_zero(),
            material: Material::default(),
        };
        t.normal = t.get_normal();
        return t;
    }
    pub fn intersect(&self, ray: &Ray) -> f64 {
        let v0v1: Vec3f = &self.v1 - &self.v0;
        let v0v2: Vec3f = &self.v2 - &self.v0;

        let pvec = cross_product(&ray.dir, &v0v2);
        let det = v0v1.dot(&pvec);

        if ((det).abs() < 0.001) {
            return f64::INFINITY;
        }
        let invDet = 1.0 / det;

        let tvec: Vec3f = &ray.origin - &self.v0;
        let u = tvec.dot(&pvec) * invDet;

        if (u < 0.0 || u > 1.0) {
            return f64::INFINITY;
        }

        let qvec = cross_product(&tvec, &v0v1);
        let v = &ray.dir.dot(&qvec) * invDet;

        if (v < 0.0 || u + v > 1.0) {
            return f64::INFINITY;
        };

        let t = v0v2.dot(&qvec) * invDet;
        if t > 0.0 {
            t
        } else {
            return f64::INFINITY;
        }
    }

    pub fn get_normal(&self) -> Vec3f {
        let v1 = &self.v1 - &self.v0;
        let v2: Vec3f = &self.v2 - &self.v0;

        return normalize(&cross_product(&v1, &v2));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disc {
    pub position: Vec3f,
    pub norm: Vec3f,
    pub radius_2: f64,
    pub material: Material,
}
impl Disc {
    pub fn intersect(&self, ray: &Ray) -> f64 {
        let denom = ray.dir.dot(&self.norm);

        if denom.abs() < 0.000001 {
            return f64::INFINITY;
        }
        let po = &self.position - &ray.origin;
        let dist = po.dot(&self.norm) / denom;

        if dist < 0.0 {
            return f64::INFINITY;
        }

        let collide = (&ray.dir * dist) + &ray.origin;

        let d = norm_vec_2(&(collide - &self.position));

        if d > self.radius_2 {
            f64::INFINITY
        } else {
            dist
        }
    }
    pub fn get_normal(&self, _loc: &Vec3f) -> Vec3f {
        self.norm.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub position: Vec3f,
    pub norm: Vec3f,
    pub material: Material,
}
impl Plane {
    pub fn intersect(&self, ray: &Ray) -> f64 {
        let denom = ray.dir.dot(&self.norm);

        if denom.abs() < 0.000001 {
            return f64::INFINITY;
        }
        let po = &self.position - &ray.origin;
        let dist = po.dot(&self.norm) / denom;

        if dist < 0.0 {
            return f64::INFINITY;
        } else {
            return dist;
        }
    }
    pub fn get_normal(&self, _loc: &Vec3f) -> Vec3f {
        self.norm.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    pub origin: Vec3f,
    pub radius: f64,
    pub material: Material,
}
impl Sphere {
    pub fn new(v: Vec3f, radius: f64, material: Material) -> Sphere {
        Sphere {
            origin: v,
            radius,
            material,
        }
    }
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
pub fn cross_product(a: &Vec3f, b: &Vec3f) -> Vec3f {
    let x = (a[1] * b[2]) - (a[2] * b[1]);
    let y = a[2] * b[0] - a[0] * b[2];
    let z = a[0] * b[1] - a[1] * b[0];

    arr1(&[x, y, z])
}

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

    let a = norm_vec_2(dir);
    let os = loc - &s.origin;
    let b = 2.0 * dir.dot(&os);
    let c = os.dot(&os) - &s.radius * &s.radius;

    let disc = b * b - 4. * a * c;
    if disc > 0.0 {
        let disc_sqrt = disc.sqrt();
        let q;
        if b < 0. {
            q = (-b - disc_sqrt) / 2.0;
        } else {
            q = (-b + disc_sqrt) / 2.0;
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

pub fn load_scene_name(filename: String) -> Scene {
    let f = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let reader = BufReader::new(f);
    let mut scene: Scene = serde_json::from_reader(reader).unwrap();

    let ob = Object::Triangle(Triangle::new(
        vec3f_zero(),
        vec3f_x(),
        vec3f_z(),
        Material::mat_color(RED),
    ));
    let so = &mut scene.objects;
    so.push(ob);

    let t2 = Triangle::new(
        vec3f_zero(),
        -vec3f_x(),
        -vec3f_z(),
        Material::mat_color(WHITE),
    );

    println!("t2 : {:?}", t2);
    so.push(Object::Triangle(t2));

    let t3 = Triangle::new(
        vec3f_zero(),
        vec3f_z(),
        -vec3f_x(),
        Material::mat_color(GREEN),
    );

    println!("t3 : {:?}", t3.normal);
    so.push(Object::Triangle(t3));

    let t4 = Triangle::new(
        vec3f_zero(),
        vec3f_z(),
        -vec3f_y(),
        Material::mat_color(GREEN),
    );

    println!("t3 : {:?}", t4.normal);
    so.push(Object::Triangle(t4));

    let mut tris = load_stl_debug("./Lowpolyring.stl".to_string());
    so.append(&mut tris);

    so.append(&mut load_stl("./Lowpolyring.stl".to_string()));
    scene
}
