use rgame::*;
use interfaces::*;

fn main() {

    let mut x_position = 200.0;
    let mut y_position = 30.0;

    set_event_handler(move |context, event| match event {
        Event::Draw => {
            context.clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
            context.draw_rectangle(x_position, y_position, 100., 100., 1.0, 0.0, 0.0, 1.0);
        }
        Event::KeyDown(key) => {
            let move_amount = 20.0;
            match key {
                Key::Left => x_position -= move_amount,
                Key::Right => x_position += move_amount,
                Key::Up => y_position += move_amount,
                Key::Down => y_position -= move_amount,
                Key::Space => {}
            }
        }
    })
}