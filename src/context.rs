extern "C" {
    fn js_clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32);
    fn js_draw_rectangle(
        x: f64,
        y: f64,
        width: f32,
        height: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    );
}

pub struct Context {}

impl Context {
    pub fn clear_screen_to_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            js_clear_screen_to_color(red, green, blue, alpha)
        }
    }
    
    pub fn draw_rectangle(
        &mut self,
        x: f64,
        y: f64,
        width: f32,
        height: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) {
        unsafe {
            js_draw_rectangle(x, y, width, height, red, green, blue, alpha);
        }
    }
}