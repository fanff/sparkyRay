#[cfg(test)]
mod tests_render {
    use image::imageops::{resize, FilterType};
    use std::fs::File;
    use std::io::BufReader;
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

        //let j = ;
        let w = match File::create("data.json") {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let j = match serde_json::to_writer_pretty(w, &scene) {
            Ok(file) => file,
            Err(error) => panic!("Problem serialising {:?}", error),
        };

        let img = scene.render(50, 50, 3, -1.0, 1.0, -1.0, 1.0);
        img.save("sphere1.png");
    }

    #[test]
    fn test_read_scene1() {
        let f = match File::open("scene1.json") {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let reader = BufReader::new(f);
        let scene: Scene = serde_json::from_reader(reader).unwrap();
        let factor = 2;
        let img = scene.render(19 * factor, 10 * factor, 5, -1.0, 1.0, -1.0, 1.0);

        let f = resize(
            &img,
            (19 * factor * 4) as u32,
            (10 * factor * 4) as u32,
            FilterType::Nearest,
        );
        f.save("scene1.png");
    }

    #[test]
    fn test_render_multi() {
        let f = match File::open("scene1.json") {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let reader = BufReader::new(f);
        let scene: Scene = serde_json::from_reader(reader).unwrap();
        let factor = 1;
        let img = scene.render(19 * factor, 10 * factor, 5, -1.0, 1.0, -1.0, 1.0);

        let f = resize(
            &img,
            (19 * factor * 4) as u32,
            (10 * factor * 4) as u32,
            FilterType::Nearest,
        );
        f.save("scene1_multi.png");
    }
}

#[cfg(test)]
mod tests_view_port {
    use rustyray::ViewZone;

    #[test]
    fn test_split_view_port() {
        let v = ViewZone::fullratio();
        let s1 = v.split_n_ratio(3, 3);

        dbg!("{:?}", s1);
    }
}
