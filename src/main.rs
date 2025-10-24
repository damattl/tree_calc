use calculate::calculate_result;
use iced::{
    Alignment, Element, Length, Size, alignment, application, color,
    widget::{Button, Column, Text, button, column, container, row, text, text_input},
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
    KeypadPressed(&'static str),
    Calculate,
    RemoveLast,
    Clear,
}

#[derive(Default)]
struct Calculator {
    input: String,
    stack: Vec<String>,
    error: Option<String>,
}

static CONSTANTS: Constants<f32> = Constants::<f32> {
    e: std::f32::consts::E,
    pi: std::f32::consts::PI,
};

pub fn keypad<Msg: Clone>(label: impl Into<String>) -> Button<'static, Msg> {
    let text = Text::new(label.into()).size(18).align_x(Alignment::Center);

    button(text).width(64).padding([12, 16])
}

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
                        self.error = None;
                        let result: f32 = calculate_result(&term);
                        self.stack.push(self.input.clone());
                        self.input = result.to_string();
                    }
                    Err(err) => self.error = Some(format!("Something went wrong: {}", err)),
                };
            }
            Message::KeypadPressed(pad) => {
                self.input = self.input.clone() + &pad;
            }
            Message::RemoveLast => {
                let mut modified = self.input.trim_end().to_string();
                modified.pop();
                self.input = modified;
            }
            Message::Clear => {
                self.input = "".to_owned();
            }
        }
    }

    pub fn width() -> f32 {
        let spacing = 4.0; // TODO: Make default
        spacing * 3.0 + 64.0 * 4.0
    }

    pub fn view(&self) -> Column<'_, Message> {
        let spacing = 4;

        let keypad = column![
            row![
                keypad("B").on_press(Message::RemoveLast),
                keypad("C").on_press(Message::Clear),
                keypad("%"),
                keypad("/"),
            ]
            .spacing(spacing),
            row![
                keypad("7").on_press(Message::KeypadPressed("7")),
                keypad("8").on_press(Message::KeypadPressed("8")),
                keypad("9").on_press(Message::KeypadPressed("9")),
                keypad("*").on_press(Message::KeypadPressed(" * ")),
            ]
            .spacing(spacing),
            row![
                keypad("4").on_press(Message::KeypadPressed("4")),
                keypad("5").on_press(Message::KeypadPressed("5")),
                keypad("6").on_press(Message::KeypadPressed("6")),
                keypad("-").on_press(Message::KeypadPressed(" - ")),
            ]
            .spacing(spacing),
            row![
                keypad("1").on_press(Message::KeypadPressed("1")),
                keypad("2").on_press(Message::KeypadPressed("2")),
                keypad("3").on_press(Message::KeypadPressed("3")),
                keypad("+").on_press(Message::KeypadPressed(" + ")),
            ]
            .spacing(spacing),
            row![
                keypad(" ").on_press(Message::KeypadPressed(" ")),
                keypad("0").on_press(Message::KeypadPressed("0")),
                keypad(",").on_press(Message::KeypadPressed(",")),
                keypad("=").on_press(Message::Calculate),
            ]
            .spacing(spacing),
        ]
        .spacing(spacing);

        let full_width = Calculator::width();

        let text_in: Element<'_, Message> = text_input("", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::Calculate)
            .width(full_width)
            .align_x(Alignment::End)
            .size(18)
            .into();

        let stack_or_error = match &self.error {
            None => {
                if self.stack.len() == 0 {
                    text("")
                } else {
                    let start = (self.stack.len() as i64 - 3).max(0) as usize;
                    let mut stack_text = String::from("");
                    for i in start..self.stack.len() {
                        let el = &self.stack[i];
                        stack_text += "    ";
                        stack_text += el;
                    }
                    text(stack_text)
                }
            }
            Some(error) => text(error.to_owned()).color(color!(0xff0000)),
        }
        .width(full_width)
        .align_x(Alignment::End)
        .size(12);

        column![text_in, container(stack_or_error).padding([4, 5]), keypad,].padding([10, 5])
    }
}

fn main() -> iced::Result {
    let app = application("Calculator", Calculator::update, Calculator::view).window_size(Size {
        width: Calculator::width() + 10.0,
        height: 360.0,
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
