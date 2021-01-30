use iced_native::Command;
use iced_remarkable::application::Application;

mod counter;

struct CounterApp {
    counter: counter::Counter,
}

impl Application for CounterApp {
    type Message = counter::Message;

    fn new() -> (Self, iced_native::Command<Self::Message>) {
        println!("CounterApp.new",);
        (
            CounterApp {
                counter: counter::Counter::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, messages: Vec<Self::Message>) -> Vec<iced_native::Command<Self::Message>> {
        println!("counterApp.update",);
        vec![]
    }

    fn view(
        &mut self,
    ) -> iced_native::Element<'_, Self::Message, iced_remarkable::RemarkableRenderer> {
        self.counter.view().into()
    }
}

fn main() {
    CounterApp::run();
}
