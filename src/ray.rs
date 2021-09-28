use crate::vec3::{Pos3, Vec3};

pub struct Ray {
    pub orig: Pos3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Pos3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> Pos3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn ray() {
        use super::{Ray, Pos3, Vec3};

        let ray = Ray::new(Pos3::new(1.0, 2.0, 3.0), Vec3::new(10.0, 15.0, 4.0));
        assert_eq!(ray.at(2.0), Pos3::new(21.0, 32.0, 11.0));
    }
}
