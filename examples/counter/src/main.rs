#![feature(nll)]
use iced_native::{
    input::mouse,
    widget::{button, Button, Column, Text},
    Cache, Element, Event, Size, UserInterface,
};
use iced_remarkable::RemarkableRenderer;
use libremarkable::framebuffer::{
    common::{DISPLAYHEIGHT, DISPLAYWIDTH},
    core::Framebuffer,
    FramebufferBase,
};

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Clone, Debug, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Counter {
    pub fn new() -> Self {
        Counter::default()
    }
    pub fn view(&mut self) -> Text {
        // We use a column: a simple vertical layout
        // Column::new()
        //     .push(
        //         // The increment button. We tell it to produce an
        //         // `IncrementPressed` message when pressed
        //         Button::new(&mut self.increment_button, Text::new("+"))
        //             .on_press(Message::IncrementPressed),
        //     )
        //     .push(
        //         // We show the value of the counter here
        //         Text::new(&self.value.to_string()).size(50),
        //     )
        //     .push(
        //         // The decrement button. We tell it to produce a
        //         // `DecrementPressed` message when pressed
        //         Button::new(&mut self.decrement_button, Text::new("-"))
        //             .on_press(Message::DecrementPressed),
        //     )
        Text::new(&self.value.to_string())
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}
fn main() {
    let mut counter = Counter::new();
    let mut cache = Cache::new();
    let mut renderer = RemarkableRenderer::new();
    let mut window_size = Size::new(DISPLAYWIDTH as f32, DISPLAYHEIGHT as f32);
    let mut i = 0;
    loop {
        let user_interface: UserInterface<Message, RemarkableRenderer> =
            UserInterface::build(counter.view(), window_size, cache, &mut renderer);
        // let messages = user_interface.update(
        //     vec![Event::Mouse(mouse::Event::CursorMoved { x: 40.0, y: 40.0 })],
        //     None,
        //     &renderer,
        // );
        // let primitive = user_interface.draw(&mut renderer);
        cache = user_interface.into_cache();
        // renderer.render(&primitive);
        i = i + 1;
        if (i > 3) {
            break;
        }
    }
}
