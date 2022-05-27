
// use rustyray::Vec3f;


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_1() {
        let v1:rustyray::Vec3f =
            rustyray::Vec3f{x:2.0,y:1.0,z:2.0};

        dbg!((v1.x, v1.y,v1.z));
    }
}