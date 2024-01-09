// pub mod widget;
pub mod lexer;

use iced::widget::{button, text};
use lexer::{Link, MarkdownLexs, MarkdownLexer};
pub use open;

pub fn view_markdown<'a, Message, Renderer>(
    markdown: &'a Vec<lexer::MarkdownLexs>,
) -> iced::Element<'a, Message, Renderer>
where 
    Message: Markdown + Clone + 'a,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
    Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::button::StyleSheet,
{
    let mut column = iced::widget::Column::new();

    let mut test = markdown.iter();

    while let Some(token) = test.next() {
        let mut row = Vec::new();

        if let MarkdownLexs::NewLine = token {
            break;
        }

        row.push(handle_token(token));

        while let Some(token) = test.next() {
            if let MarkdownLexs::NewLine = token {
                break;
            }

            row.push(handle_token(token));
        }

        column = column.push(iced::widget::Row::with_children(row));

        
    }

    column.into()
}

pub trait Markdown {
    fn handle_url(url: String) -> Self;
}

fn handle_token<'a, Message, Renderer>(
    token: &'a lexer::MarkdownLexs,
) -> iced::Element<'a, Message, Renderer>
where 
    Message: Markdown + Clone + 'a,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
    Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::button::StyleSheet,
{
    match token {
        MarkdownLexs::Text(c) => {
            return iced::widget::Text::new(c).into();
        }
        MarkdownLexs::Heading(_heading, heading) => {
            return iced::widget::Text::new(heading).into();
        }
        MarkdownLexs::Link(link) => {
            let Link { content, url } = link;

            return iced::widget::Button::new(
                content
                    .iter()
                    .fold(iced::widget::Row::new(), |output, lex| {
                        output.push(handle_token(lex))
                    }),
            )
            .on_press(Message::handle_url(url.clone()))
            .into();
        }
        MarkdownLexs::NewLine => panic!("NewLine"),
        MarkdownLexs::Image(_) => {
            return iced::widget::Text::new("Image").into();
        }
    }
}

#[test]
fn test_parse_heading() {
    use crate::lexer::MarkdownLexer;
    use std::io::Read;

    let mut buf = String::new();
    let _file = std::fs::File::open("roadmap.md")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let mut parser = MarkdownLexer::new(&buf);
    parser.parse_text();
    let tokens = parser.output();

    println!("{:?}", tokens);
}
