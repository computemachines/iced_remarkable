use iced_native::{
    widget::{button, Button, Column, Text},
    Cache, Size, UserInterface,
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
    increment: button::State,
    decrement: button::State,
}

#[derive(Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }
    fn view(&mut self) -> Button<Message, RemarkableRenderer> {
        dbg!(self.value);
        Button::new(&mut self.increment, Text::new("Increment")) //.on_press(Message::IncrementPressed)
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                println!("Pressed increment")
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
    let mut framebuffer = Framebuffer::new("/dev/fb0");
    let mut window_size = Size::new(DISPLAYWIDTH as f32, DISPLAYHEIGHT as f32);

    let mut user_interface =
        UserInterface::build(counter.view(), window_size, cache, &mut renderer);
    let primitive = user_interface.draw(&mut renderer);
    renderer.render(&mut framebuffer, &primitive);
}
