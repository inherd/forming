use crate::parser;

use pulldown_cmark::{
    CodeBlockKind,
    Event::{Code, End, Start, Text},
    Options, Parser, Tag,
};

pub struct Wmd {
    text: String,
}

impl Wmd {
    pub fn new(text: String) -> Wmd {
        Wmd { text }
    }

    pub fn parse(&mut self) {
        let parser = create_markdown_parser(&self.text);
        let mut text = "".to_string();

        for event in parser {
            match event {
                Start(Tag::CodeBlock(info)) => {
                    match info {
                        CodeBlockKind::Fenced(lang_code) => {
                            let string = lang_code.to_string();
                            println!("{}", string);
                        }
                        CodeBlockKind::Indented => {}
                    }

                    text = "".to_string();
                }
                End(Tag::CodeBlock(info)) => {
                    match info {
                        CodeBlockKind::Fenced(_lang_code) => {

                        }
                        CodeBlockKind::Indented => {}
                    }
                }
                Text(body) => {
                    let str = body.to_string();

                    if str.starts_with("// doc-") {
                        let writing = parser::parse(str.replace("//", "").as_str());
                        println!("{:?}", writing);
                    }
                }
                Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }
    }
}

fn create_markdown_parser(content: &str) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    Parser::new_ext(&content, options)
}

#[cfg(test)]
mod build_command_structure {
    use super::*;

    #[test]
    fn should_parse_line() {
        let mut rmd = Wmd::new("
```java
// doc-code: file(\"src/lib.rs\").line()[2, 5]
```
".to_string());
        rmd.parse();
    }
}
