#[cfg(test)]
mod tests {
    // use super::*;
    use rustyray::{
        intersect_sphere, norm_vec, norm_vec_2, rotation_matrix, Camera, Color, Light, Material,
        Object, Ray, Scene, Sphere, Vec3f, BLACK, RED, WHITE,
    };

    use ndarray::{arr1, arr2, Array1, Array2};

    #[test]
    fn test_1() {
        let v1: &Vec3f = &arr1(&[2., 0., 0.]);

        // rustyray::Vec3f()
        dbg!(v1);

        assert_eq!(4.0, norm_vec_2(v1));
        assert_eq!(2.0, norm_vec(&v1));

        let rm = rotation_matrix(v1, 1.0);
    }

    #[test]
    fn test_sphereIntersect() {
        let origin: Vec3f = arr1(&[0., 0., 0.]);

        let dir: Vec3f = arr1(&[1., 0., 0.]);

        let s1 = Sphere {
            origin: arr1(&[5., 0., 0.]),
            radius: 2.0,

            material: Material {
                reflection: 0.5,
                color: RED,
                diffuse_c: 0.75,
                specular_c: 0.5,
                specular_k: 10.0,
            },
        };

        let light = Light {
            color: WHITE,
            pos: arr1(&[0., 0., 10.]),
        };

        let scene = Scene {
            camera: Camera {
                origin: origin.clone(),
                dir: dir.clone(),
            },
            objects: vec![Object::Sphere(s1)],
            lights: vec![light],
        };

        let ray = Ray {
            origin: origin.clone(),
            dir: dir.clone(),
        };

        println!("{:#?}", scene.objects[0].intersect(&ray));
        println!("{:#?}", scene.trace_ray(&ray));
        let color = scene.raycalc(&ray, 3);

        scene.render(500, 500);
    }
}
