

#[cfg(test)]
mod tests {
    // use super::*;
    use rustyray::{norm_vec_2,norm_vec, Vec3f, rotation_matrix,
                   Sphere,intersect_sphere,Color};

    use ndarray::{arr1, arr2, Array1, Array2};


    #[test]
    fn test_1() {
        let v1: &Vec3f =
            &arr1(&[2., 0., 0.]);

        // rustyray::Vec3f()
        dbg!(v1);

        assert_eq!(4.0, norm_vec_2(v1));
        assert_eq!(2.0, norm_vec(&v1));


        let rm = rotation_matrix(v1, 1.0);
    }

    #[test]
    fn test_sphereIntersect() {
        let loc: &Vec3f =
            &arr1(&[0., 0., 0.]);

        let dir: &Vec3f =
            &arr1(&[1., 0., 0.]);

        let s1 = Sphere{origin:arr1(&[5., 0., 0.]) , radius :2.0,
                        color:Color::new(1.,2.,3.)};

        let dist = intersect_sphere(&loc, &dir, &s1);
        assert_eq!(3.0, dist);
    }
}