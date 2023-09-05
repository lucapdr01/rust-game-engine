use rgame::{scene::Scene, kinematics::{game_object::GameObject, vector3::Vector3}};

fn main() {
    //Init Game Environment
    let mut scene = Scene::init();

    //Create a player
    let start_pos = Vector3::new(10.0, 10.0, 0.0);
    let player = GameObject::new("main".to_owned(), Some(start_pos));
    
    //add player to the environment
    scene.add(player);
}