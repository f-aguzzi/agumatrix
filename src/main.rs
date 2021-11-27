use iced::{Sandbox, Align, Element, button, text_input, Column, Text, Settings};

mod lib;
use lib::parse;


#[derive(Default, Debug, Clone)]
struct Agumatrix {
    determinant: f64,
    matrix: String,
    calculate_button: button::State,
    input: text_input::State
}

#[derive(Debug, Clone)]
enum Message {
    CalculateButtonPressed,
    TextInputChanged(String)
}

impl Sandbox for Agumatrix {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CalculateButtonPressed => {
                self.determinant = parse(self.matrix.clone()).deter()
            }
            Message::TextInputChanged(value) => {
                self.matrix = value
            }
        }
    }

    fn title(&self) -> String {
        String::from("AguMatrix")
    }

    fn view(&mut self) -> Element<Self::Message> {
        
        Column::new()
        .padding(30)
        .align_items(Align::Center)
        .push(
            Text::new("AguMatrix")
            .size(30)
            .color([255.0 ,85.0 ,0.0])
        )
        .push(
            text_input::TextInput::new(&mut self.input, "Write your matrix here", &self.matrix, Message::TextInputChanged)
        )
        .push(
            button::Button::new(&mut self.calculate_button, Text::new("Calculate determinant"))
                .on_press(Message::CalculateButtonPressed)
                .padding(12)
        )
        .push(
            Text::new(self.determinant.to_string()).size(20)
        )
        .into()
    }
}


fn main() {
    Agumatrix::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
