use calculate::calculate_result;
use iced::{
    Alignment, Element, Size, application, color,
    widget::{Column, button, column, row, text, text_input},
};
use parse::parse_term;

use crate::traits::Constants;

mod calculate;
mod parse;
mod token;
mod traits;
mod tree;

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Calculate,
}

#[derive(Default)]
struct Calculator {
    input: String,
    output: String,
    error: String,
}

static CONSTANTS: Constants<f32> = Constants::<f32> {
    e: std::f32::consts::E,
    pi: std::f32::consts::PI,
};

impl Calculator {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
            }
            Message::Calculate => {
                let term = parse_term(&self.input, &CONSTANTS);
                match term {
                    Ok(term) => {
                        self.error = "".to_owned();
                        let result: f32 = calculate_result(&term);
                        self.output = result.to_string();
                    }
                    Err(err) => self.error = format!("Something went wrong: {}", err),
                };
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let text_in: Element<'_, Message> = text_input("", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::Calculate)
            .into();
        let calc_row = row![
            text_in,
            button("=").on_press(Message::Calculate),
            text(&self.output)
                .width(100)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        ];

        let error = text(&self.error).color(color!(0xff0000));

        column![calc_row, error]
    }
}

fn main() -> iced::Result {
    let app = application("Calculator", Calculator::update, Calculator::view).window_size(Size {
        width: 300.0,
        height: 300.0,
    });
    app.run()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_basic_operations() {
        let term_tree = parse_term("(3 + 4) * (17 + 3 * -4 + 5) =", &CONSTANTS).unwrap();
        term_tree.traverse_postorder(0, &mut |node| println!("{node:?}"));
        println!("{:#?}", term_tree);
        let result: f32 = calculate_result(&term_tree);
        println!("{result:?}");

        assert_eq!(result as i32, (3 + 4) * (17 + 3 * -4 + 5));
    }

    #[test]
    fn test_minus() {
        let second_term = parse_term("3 - 4", &CONSTANTS).unwrap();

        let result: f32 = calculate_result(&second_term);
        println!("{result:?}");

        assert_eq!(result as i32, 3 - 4);
    }
}
