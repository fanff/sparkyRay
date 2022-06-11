use crate::{Material, Object, Sphere, Triangle, Vec3f, BLUE};
use ndarray::arr1;
use stl_io::{Vector, Vertex};

pub fn vertex_to_Vec(v: &Vector<f32>) -> Vec3f {
    let x = (v[0]) as f64;
    let y = (v[1]) as f64;
    let z = (v[2]) as f64;
    let r = arr1(&[x, y, z]);

    r
}

pub fn load_stl(name: String) -> Vec<Object> {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().read(true).open(name).unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();

    let alltris = stl.faces.iter().map(|f| {
        let v = f.vertices;

        let p1 = stl.vertices.get(v[0]).unwrap();
        let p2 = stl.vertices.get(v[1]).unwrap();
        let p3 = stl.vertices.get(v[2]).unwrap();
        let pp1 = vertex_to_Vec(p1) * 1000.0;
        let pp2 = vertex_to_Vec(p2) * 1000.0;
        let pp3 = vertex_to_Vec(p3) * 1000.0;

        //println!("{:?} ,{:?} ,{:?}, {:?}", v, pp1, pp2, pp3);
        let mut t = Triangle::new(pp1, pp2, pp3, Material::mat_color(BLUE));

        t.normal = vertex_to_Vec(&f.normal);
        Object::Triangle(t)
    });
    let a: Vec<Object> = alltris.take(128).collect();
    println!("{:?}", a.len());
    a
}

pub fn load_stl_debug(name: String) -> Vec<Object> {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().read(true).open(name).unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();

    let alltris = stl.vertices.iter().map(|f| {
        let pp1 = vertex_to_Vec(f) * 1000.0;

        println!("{:?} ,{:?} ", f, pp1);
        let t = Sphere::new(pp1, 0.3, Material::mat_color(BLUE));

        Object::Sphere(t)
    });
    let a: Vec<Object> = alltris.take(8).collect();
    println!("{:?}", a.len());
    a
}
