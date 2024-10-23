use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }
}
