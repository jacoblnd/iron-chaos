use iced::widget::{column, text, button, Column};
use iced::Theme;

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

fn theme(state: &Counter) -> Theme {
    if state.value > 1 {
        Theme::TokyoNight
    } else {
        Theme::KanagawaDragon
    }
}

impl Counter {
    fn view(&self) -> Column<Message> {
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);
        let counter = text(self.value);
        let interface = column![increment, counter, decrement];
        interface
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            },
            Message::Decrement => {
                self.value -= 1;
            },
        }
    }
}

fn main() -> iced::Result {
    iced::application("A cool counter", Counter::update, Counter::view)
        .theme(theme)
        .run()
}
