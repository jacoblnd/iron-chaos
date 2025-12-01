use iced::widget::{Container, Row, button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Border, Color, Element, Length, Size, Theme};

use iron_chaos::rbn::{self, RBN};

const INITIAL_RBN_N: &'static str = "20";
const INITIAL_RBN_K: &'static str = "2";
const INITIAL_WINDOW_WIDTH: f32 = 1000.0;
const INITIAL_WINDOW_HEIGHT: f32 = 800.0;
const INITIAL_WINDOW_SIZE: Size = Size::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

#[derive(Debug, Clone)]
enum Message {
    Advance,
    NewRBN,
    RBNParamNChanged(String),
    RBNParamKChanged(String),
}

struct Controller {
    rbn: rbn::SynchronousRBN,
    matrix_data: Vec<Vec<u8>>,
    text_input_rbn_param_n: String,
    text_input_rbn_param_k: String,
    window_size: Size,
    warning_text: String,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            rbn: rbn::SynchronousRBN::default(),
            matrix_data: Vec::default(),
            text_input_rbn_param_n: INITIAL_RBN_N.to_string(),
            text_input_rbn_param_k: INITIAL_RBN_K.to_string(),
            window_size: INITIAL_WINDOW_SIZE,
            warning_text: "".to_string(),
        }
    }
}

fn theme(_: &Controller) -> Theme {
    Theme::TokyoNight
}

impl Controller {
    fn view(&self) -> Row<'_, Message> {
        let rbn_view_width = self.window_size.width * (2.0 / 3.0);
        let right_pane_width = self.window_size.width * (1.0 / 3.0);
        // println!("Window Size: {}", self.window_size.width);
        // println!("Matrix width: {}", rbn_view_width);
        // println!("Right Pane width: {}", right_pane_width);
        let left_rbn_container =
            container(scrollable(self.rbn_container(&self.matrix_data))).width(rbn_view_width);
        let right_config_container = self.config_container().width(right_pane_width);
        let interface = row![left_rbn_container, right_config_container,];
        interface
    }
    fn update(&mut self, message: Message) {
        self.clear_warning();
        match message {
            Message::Advance => {
                for _ in 0..20 {
                    self.matrix_data.push(self.rbn.advance(1));
                }
            }
            Message::NewRBN => {
                let parsed_n: usize = match self.text_input_rbn_param_n.parse() {
                    Ok(num) => num,
                    Err(e) => {
                        self.warning_text = format!(
                            "Could not parse {} into integer {}",
                            self.text_input_rbn_param_n, e
                        );
                        return;
                    }
                };
                let parsed_k: usize = match self.text_input_rbn_param_k.parse() {
                    Ok(num) => num,
                    Err(e) => {
                        self.warning_text = format!(
                            "Could not parse {} into integer {}",
                            self.text_input_rbn_param_k, e
                        );
                        return;
                    }
                };
                self.rbn = rbn::SynchronousRBN::new(parsed_n, parsed_k, 0.8);
                self.rbn.rand_activate_nodes(0.1);
                self.matrix_data = vec![self.rbn.advance(1)];
            }
            Message::RBNParamNChanged(new_value) => {
                self.text_input_rbn_param_n = new_value;
            }
            Message::RBNParamKChanged(new_value) => {
                self.text_input_rbn_param_k = new_value;
            }
        }
    }

    fn clear_warning(&mut self) {
        self.warning_text = "".to_string();
    }

    fn config_container(&self) -> Container<'_, Message> {
        let advance = button("Advance").on_press(Message::Advance);
        let new_rbn = button("New RBN").on_press(Message::NewRBN);
        let text_input_rbn_param_n: Element<Message> = container(
            text_input("5", &self.text_input_rbn_param_n).on_input(Message::RBNParamNChanged),
        )
        .max_width(100)
        .into();
        let text_input_rbn_param_k: Element<Message> = container(
            text_input("2", &self.text_input_rbn_param_k).on_input(Message::RBNParamKChanged),
        )
        .max_width(100)
        .into();
        container(
            column![
                row![advance, new_rbn].spacing(10),
                text("RBN Param N:"),
                text_input_rbn_param_n,
                text("RBN Param K"),
                text_input_rbn_param_k,
                text(&self.warning_text).color(RED)
            ]
            .align_x(Alignment::Center)
            .spacing(10),
        )
    }

    fn rbn_container(&self, data: &Vec<Vec<u8>>) -> Element<'_, Message> {
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
    iced::application("Iron Chaos", Controller::update, Controller::view)
        .theme(theme)
        .window_size(INITIAL_WINDOW_SIZE)
        .run()
}
