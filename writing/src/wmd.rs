use pulldown_cmark::{
    CodeBlockKind,
    Event::{Code, End, Html, Start, Text},
    Options, Parser, Tag,
};

pub struct Rmd {
    text: String,
}

impl Rmd {
    pub fn new(text: String) -> Rmd {
        Rmd { text }
    }

    pub fn parse(&mut self)  {
        let parser = create_markdown_parser(&self.text);
        let mut text = "".to_string();

        for event in parser {
            match event {
                Start(tag) => {
                    if let Tag::CodeBlock(info) = tag {
                        match info {
                            CodeBlockKind::Fenced(lang_code) => {
                                let string = lang_code.to_string();
                                println!("{}", string);
                            }
                            CodeBlockKind::Indented => {}
                        }
                    }

                    text = "".to_string();
                }
                End(tag) => {
                    if let Tag::CodeBlock(info) = tag {
                        match info {
                            CodeBlockKind::Fenced(_lang_code) => {

                            }
                            CodeBlockKind::Indented => {}
                        }
                    }
                }
                Text(body) => {
                    text += &body.to_string();
                }
                Html(_html) => {}
                Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                }
                _ => (),
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
        let mut rmd = Rmd::new("
```java
// doc-code: file(\"src/lib.rs\").line()[2, 5]
```
".to_string());
        rmd.parse();
    }

}
