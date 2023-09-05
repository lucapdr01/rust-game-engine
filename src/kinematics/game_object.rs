use crate::kinematics::vector3::*;

struct GameObject {
    name : String,
    position: Vector3,
}

impl GameObject{
   fn new(name: String, position: Option<Vector3>) -> GameObject{

    let position = match position {
        Some(pos) => pos,
        None => Vector3::new(0.0, 0.0, 0.0),
    };
    GameObject { name, position }
   }

   fn translate(&mut self, destination: Vector3){
    //TODO support linear update of render
    self.position = destination;
   }

   //TODO impl rotate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game_obj = GameObject::new("test".to_owned(),None);
        assert_eq!(game_obj.name, "test");
    }

    #[test]
    fn test_translate(){
        let mut game_obj = GameObject::new("test".to_owned(),None);
        let dest = Vector3::new(1.0,1.0,1.0);
        game_obj.translate(dest);

        assert_eq!(game_obj.position.x, 1.0);
        assert_eq!(game_obj.position.y, 1.0);
        assert_eq!(game_obj.position.z, 1.0);
        
    }
}


