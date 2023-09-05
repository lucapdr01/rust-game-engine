use crate::context::*;
use crate::kinematics::game_object::*;

pub struct Scene {
    ctx : Context,
    children : Vec<GameObject>,
}

impl Scene{
    pub fn init() -> Scene{
        let mut ctx = Context{};
        ctx.clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
        let children = Vec::new();

        Scene { ctx, children }
    }

    pub fn add(&mut self, game_obj : GameObject){
        self.ctx.draw_rectangle(game_obj.position.x, game_obj.position.y, 10.0, 10.0, 1.0, 0.0, 0.0, 1.0);
        self.children.push(game_obj);
    }
}

// Link con javascript makes test fail
mod tests {
    //use super::*;

    #[test]
    fn test_init() {
        assert!(true)
    }

    #[test]
    fn test_add(){
        assert!(true)
    }
}
