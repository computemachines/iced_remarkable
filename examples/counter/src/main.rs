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
    // decrement: button::State,
}

#[derive(Clone)]
enum Message {
    IncrementPressed,
    // DecrementPressed,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }
    fn view(&mut self) -> Element<Message, RemarkableRenderer> {
        dbg!(self.value);
        Button::new(&mut self.increment_button, Text::new("Increment")).into() //.on_press(Message::IncrementPressed)
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                println!("Pressed increment")
            } // Message::DecrementPressed => {
              //     self.value -= 1;
              // }
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    let mut cache = Cache::new();
    let mut renderer = RemarkableRenderer::new();
    let window_size = Size::new(DISPLAYWIDTH as f32, DISPLAYHEIGHT as f32);

    loop {
        let root = counter.view();
        let user_interface = UserInterface::build(root, window_size, cache, &mut renderer);
        // let messages = user_interface.update(
        //     vec![Event::Mouse(mouse::Event::CursorMoved { x: 40.0, y: 40.0 })],
        //     None,
        //     &renderer,
        // );
        // let primitive = user_interface.draw(&mut renderer);
        cache = user_interface.into_cache();
        // renderer.render(&primitive);
    }
}
