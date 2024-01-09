use std::io::Read;

use iced::{widget::text, Application};
use iced_markdown::{lexer::{MarkdownLexer, MarkdownLexs}, Markdown};

fn main() -> Result<(), iced::Error> {
    App::run(iced::Settings::default())
}

struct App {
    markdown: Vec<MarkdownLexs>,
}

#[derive(Debug, Clone)]
enum Message {
    Url(String),
}

impl Markdown for Message {
    fn handle_url(url: String) -> Self {
        Message::Url(url)
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let mut buf = String::new();
        let _file = std::fs::File::open("roadmap.md")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();

        let mut parser = MarkdownLexer::new(&buf);
        parser.parse_text();
        let tokens = parser.output();

        (
            App {
                markdown: tokens,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Markdown")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Url(url) => {
                iced_markdown::open::that(url).unwrap();
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        iced_markdown::view_markdown(&self.markdown)
    }
}
