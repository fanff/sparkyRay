
// use rustyray::Vec3f;
mod rustyray;

use rustyray::norm_vec_2;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_1() {
        let v1:rustyray::Vec3f =
            rustyray::Vec3f{x:1.0,y:0.0,z:0.0};

        dbg!((v1.x, v1.y,v1.z));


         assert_eq!(1.0, norm_vec_2(v1));

    }
}