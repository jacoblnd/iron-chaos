use iced::widget::{Column, Row, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Theme};

use iron_chaos::rbn::{self, RBN};

const INITIAL_RBN_SIZE: &'static str = "20";

#[derive(Debug, Clone)]
enum Message {
    Advance,
    NewRBN,
    RBNSizeChanged(String),
}

struct Controller {
    rbn: rbn::SynchronousRBN,
    matrix_data: Vec<Vec<u8>>,
    input_rbn_size: String,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            input_rbn_size: INITIAL_RBN_SIZE.to_string(),
            matrix_data: Vec::default(),
            rbn: rbn::SynchronousRBN::default(),
        }
    }
}

fn theme(_: &Controller) -> Theme {
    Theme::TokyoNight
}

impl Controller {
    fn view(&self) -> Row<'_, Message> {
        let advance = button("Advance").on_press(Message::Advance);
        let new_rbn = button("New RBN").on_press(Message::NewRBN);
        let input_rbn_size: Element<Message> =
            container(text_input("5", &self.input_rbn_size).on_input(Message::RBNSizeChanged))
                .max_width(100)
                .into();
        let matrix_container = self.matrix(&self.matrix_data);
        let right_bar = column![
            column![
                row![advance, new_rbn].spacing(10),
                text("RBN Size:"),
                input_rbn_size,
            ]
            .align_x(Alignment::End)
            .spacing(10)
        ]
        .max_width(300)
        .align_x(Alignment::Center);
        let interface = row![scrollable(matrix_container), right_bar,];
        interface
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Advance => {
                self.matrix_data.push(self.rbn.advance(1));
            }
            Message::NewRBN => match self.input_rbn_size.parse() {
                Ok(parsed_value) => {
                    self.rbn = rbn::SynchronousRBN::new(parsed_value, 2, 0.5);
                    self.matrix_data = vec![self.rbn.advance(1)];
                }
                Err(_) => panic!("Could not parse input rbn size"),
            },
            Message::RBNSizeChanged(new_value) => {
                self.input_rbn_size = new_value;
            }
        }
    }

    fn matrix(&self, data: &Vec<Vec<u8>>) -> Element<'_, Message> {
        let mut column_view = column![].spacing(2).align_x(Alignment::Start);

        for row_data in data {
            let mut row_view = row![].spacing(2).align_y(Alignment::Center);
            for &value in row_data {
                let color = if value == 1 {
                    Color::BLACK
                } else {
                    Color::WHITE
                };

                let cell = container("")
                    .width(Length::Fixed(10.0))
                    .height(Length::Fixed(10.0))
                    .style(move |_theme| container::Style {
                        background: Some(color.into()),
                        border: Border {
                            color: Color::from_rgb8(200, 200, 200),
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        text_color: None,
                        ..Default::default()
                    });
                row_view = row_view.push(cell);
            }
            column_view = column_view.push(row_view);
        }
        column_view.into()
    }
}

fn main() -> iced::Result {
    iced::application("A cool counter", Controller::update, Controller::view)
        .theme(theme)
        .run()
}
