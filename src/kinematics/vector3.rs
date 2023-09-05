use std::f64;

struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3{
    fn new(x: f64, y: f64, z: f64) -> Vector3{
        let vec = Vector3 {
            x,
            y,
            z,
        };
        return vec;
    }

    fn distance(&self, vec2 : &Vector3) -> u64 {
        let dx = f64::abs(self.x - vec2.x);
        let dy = f64::abs(self.y - vec2.y);
        let dz = f64::abs(self.z - vec2.z);

        let distance = f64::sqrt(dx*dx + dy*dy + dz*dz);
        return distance as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec = Vector3::new(10.0, 20.0, 30.0);

        // Check if the values of x, y, and z are as expected
        assert_eq!(vec.x, 10.0);
        assert_eq!(vec.y, 20.0);
        assert_eq!(vec.z, 30.0);
    }

    #[test]
    fn check_distance(){
        let vec1 = Vector3::new(0.0,0.0,0.0);
        let vec2 = Vector3::new(0.0,10.0,0.0);
        let distance = vec1.distance(&vec2);
        assert!(distance == 10, "distance should be 10");
    }
}


