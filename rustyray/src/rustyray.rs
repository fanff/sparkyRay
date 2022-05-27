
pub struct Vec3f{
    pub x: f32,
    pub y: f32,
    pub z: f32
}


pub fn add_vec(vec1:Vec3f,vec2: Vec3f)-> Vec3f{
    // add two vector
    return Vec3f{x : vec1.x+vec2.x,
                y : vec1.y+vec2.y,
                z : vec1.z+vec2.z
    }
}

pub fn norm_vec_2(v:Vec3f) -> f32 {
    return v.x.powf(2.0) + v.y.powf(2.0) + v.z.powf(2.0)
}
