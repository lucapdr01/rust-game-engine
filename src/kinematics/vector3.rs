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

    fn distance(&self, vec2 : &Vector3) -> f64 {
        let dx = f64::abs(self.x - vec2.x);
        let dy = f64::abs(self.y - vec2.y);
        let dz = f64::abs(self.z - vec2.z);

        let distance = f64::sqrt(dx*dx + dy*dy + dz*dz);
        return distance as f64;
    }

    fn magnitude(&self) -> f64 {
        let vec_zero = Vector3::new(0.0,0.0,0.0);
        let magnitude = self.distance(&vec_zero);
        return  magnitude;
    }

    fn normalized(&self) -> Vector3 {
        let magnitude = self.magnitude() as f64;
        let normal_vec = Vector3::new(self.x/magnitude,self.y/magnitude,self.z/magnitude);
        return  normal_vec;
    }
    fn to_string(&self) -> String {
        let message = format!("Vector ({}, {}, {})", self.x, self.y, self.z);
        return message;
    }

    //TODO Dot and Cross Products, Sum and Difference
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
    fn test_distance(){
        let vec1 = Vector3::new(0.0,0.0,0.0);
        let vec2 = Vector3::new(0.0,10.0,0.0);
        let distance = vec1.distance(&vec2);
        assert!(distance == 10.0, "distance should be 10");
    }

    #[test]
    fn test_magnitude(){
        let vec1 = Vector3::new(0.0,-1.0,0.0);
        let magnitude = vec1.magnitude();
        assert!(magnitude >= 0.0);
        assert_eq!(magnitude, 1.0, "Magnitude should be 1");
    }

    #[test]
    fn test_normalized(){
        let vec = Vector3::new(2.0, 2.0, 2.0);
        let vec_norm = vec.normalized();
        let norm_magnitude = vec_norm.magnitude();
        assert_eq!(norm_magnitude,1.0,"Not normalized")
    }

    #[test]
    fn test_to_string(){
        let vec = Vector3::new(2.1, 2.1, 2.1);
        let mes = vec.to_string();
        assert_eq!(mes,"Vector (2.1, 2.1, 2.1)", "Not converted to string correctly")
    }

}


