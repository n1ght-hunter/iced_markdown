#[derive(Debug)]
pub enum MarkdownLexs {
    Heading(u8, String),
    Text(String),
    Link(Link),
    Image(String),
    NewLine,
}

#[derive(Debug)]
pub struct Link {
    pub(crate) content: Vec<MarkdownLexs>,
    pub(crate) url: String,
}

pub struct MarkdownLexer<'a> {
    input: &'a str,
    lexs: Vec<MarkdownLexs>,
}

impl<'a> MarkdownLexer<'a> {
    pub fn new(input: &'a str) -> Self {

        Self {
            input,
            lexs: Vec::new(),
        }
    }

    pub fn output(self) -> Vec<MarkdownLexs> {
        self.lexs
    }

    pub fn parse_text(&mut self) {
        let mut chars: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'_>>> =
            self.input.chars().enumerate().peekable();
        // let buf = Vec::new();


        while let Some((_index, char)) = chars.next() {
            self.lexs.push(parse(char, &mut chars));
        }
    }
}

fn parse(
    char: char,
    chars: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'_>>>,
) -> MarkdownLexs {
    match char {
        '#' => {
            let mut count = 1;
            let mut heading = String::new();
            while let Some((_, c)) = chars.peek() {
                match c {
                    '#' => {
                        count += 1;
                        chars.next();
                    }
                    '\n' => {
                        break;
                    }
                    c => {
                        heading.push(*c);
                        chars.next();
                    }
                }
            }

            return MarkdownLexs::Heading(count, heading);
        }
        '[' => {
            let mut content = Vec::new();
            let mut url = String::new();
            while let Some((_, c)) = chars.next() {
                match c {
                    ']' => {
                        while let Some((_, c)) = chars.next() {
                            match c {
                                '(' => {
                                    while let Some((_, c)) = chars.next() {
                                        match c {
                                            ')' => {
                                                break;
                                            }
                                            c => {
                                                url.push(c);
                                            }
                                        }
                                    }
                                    break;
                                }
                                c => {
                                    content.push(MarkdownLexs::Text(c.to_string()));
                                }
                            }
                        }
                        break;
                    }
                    c => {
                        content.push(handle_text(c, chars));
                    }
                }
            }

            return MarkdownLexs::Link(Link { content, url });
        }
        '\n' => {
            return MarkdownLexs::NewLine;
        }
        _ => {
            return handle_text(char, chars);
        },
    }
}

fn handle_text(
    char: char,
    chars: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'_>>>,
) -> MarkdownLexs {
    let mut text = String::new();
    text.push(char);
    while let Some((_, c)) = chars.peek() {
        match c {
            '#' | '[' | ']' | '\n' => {
                break;
            }
            
            c => {
                text.push(*c);
                chars.next();
            }
        }
    }

    MarkdownLexs::Text(text)
}
