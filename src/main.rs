use iced::Theme;
use iced::widget::{Column, button, column, text};

use iron_chaos::rbn::{self, RBN};

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
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> iced::Result {
    let mut s_rbn = rbn::SequentialRBN::new(5, 2, 0.5);
    s_rbn.rand_activate(0.5);
    dbg!(s_rbn);
    // iced::application("A cool counter", Counter::update, Counter::view)
    //     .theme(theme)
    //     .run()
    todo!()
}
