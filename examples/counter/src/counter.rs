use iced_native::{button, Button, Column, Text};
use iced_remarkable::RemarkableRenderer;

#[derive(Default)]
pub struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Clone, Debug, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Counter {
    pub fn new() -> Self {
        Counter::default()
    }
    pub fn view(&mut self) -> Column<Message, RemarkableRenderer> {
        // We use a column: a simple vertical layout
        Column::new()
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                // We show the value of the counter here
                Text::new(&self.value.to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
        // Text::new(&self.value.to_string()).size(10)
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
