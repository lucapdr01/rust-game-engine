pub mod interfaces;
pub mod context;
pub mod kinematics;
pub mod scene;

use interfaces::*;
use context::*;


#[no_mangle]
pub extern "C" fn key_pressed(value: usize) {
    let key = match value {
        1 => Key::Left,
        2 => Key::Right,
        3 => Key::Up,
        4 => Key::Down,
        5 => Key::Space,
        _ => return,
    };

    send_event(Event::KeyDown(key));
}

#[no_mangle]
pub extern "C" fn animate() {
    send_event(Event::Draw);
}

thread_local! {
    pub static EVENT_HANDLER_AND_CONTEXT: std::cell::RefCell<(Box<dyn FnMut(&mut Context, Event)>, Context)>
     = std::cell::RefCell::new((Box::new(|_, _|{}), Context {}));
}

pub fn set_event_handler(function: impl FnMut(&mut Context, Event) + 'static) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context| {
        // Note we're storing our `EVENT_HANDLER_AND_CONTEXT`'s internal data as a tuple of two elements.
        // To access the first item in the tuple we use the `.0` syntax.
        event_handler_and_context.borrow_mut().0 = Box::new(function);
    });
}

fn send_event(event: Event) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context| {
        let mut borrow = event_handler_and_context.borrow_mut();
        let (event_handler, context) = &mut *borrow;
        (event_handler)(context, event)
    })
}


