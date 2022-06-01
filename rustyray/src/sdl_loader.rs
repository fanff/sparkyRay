use crate::{Material, Object, Triangle, Vec3f, BLUE};
use ndarray::arr1;
use serde_json::Value::Array;
use stl_io::Vertex;

pub fn vertex_to_Vec(v: &Vertex) -> Vec3f {
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
        let pp1 = vertex_to_Vec(p1);
        let pp2 = vertex_to_Vec(p2);
        let pp3 = vertex_to_Vec(p3);

        Object::Triangle(Triangle::new(pp1, pp2, pp3, Material::mat_color(BLUE)))
    });
    let a: Vec<Object> = alltris.take(4).collect();
    println!("{:?}", a.len());
    a
}
