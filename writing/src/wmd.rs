use crate::parser;

use pulldown_cmark::{Event::{Code, End, Start, Text}, Options, Parser, Tag, CodeBlockKind, Event};
use crate::wreader::WReader;

pub struct Wmd {
    text: String,
}

impl Wmd {
    pub fn new(text: String) -> Wmd {
        Wmd { text }
    }

    pub fn parse(&mut self) -> String {
        let mut parser = create_markdown_parser(&self.text).into_offset_iter();
        let mut text = "".to_string();
        let mut is_in_code = false;
        let mut lang_code = "".to_string();

        while let Some((event, _offset)) = parser.next() {
            match event {
                Start(Tag::CodeBlock(info)) => {
                    match info {
                        CodeBlockKind::Fenced(lang) => {
                            lang_code = format!("{}", lang);
                        }
                        CodeBlockKind::Indented => {}
                    }
                    is_in_code = true;
                }
                End(Tag::CodeBlock(_info)) => {
                    is_in_code = false;
                }
                Text(body) => {
                    let str = body.to_string();

                    if is_in_code && str.starts_with("// doc-") {
                        let writing = parser::parse(str.replace("//", "").as_str());
                        let result = WReader::read_doc_code(writing.code_docs[0].clone());

                        text += &format!("```{}\n", lang_code);
                        for line in result {
                            text += &format!("{}\n", line);
                        }
                        text += &format!("```");
                    } else {
                        text += &format!("{}\n", str);
                    }
                }
                Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                }
                Event::Html(html) => {
                    text += &format!("{}\n", html);
                }
                Event::FootnoteReference(footnote) => {
                    text += &format!("{}\n", footnote);
                }
                Event::SoftBreak => {}
                Event::HardBreak => {}
                Event::Rule => {}
                Event::TaskListMarker(_) => {}
                _ => {}
            }
        }

        return text;
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
233333

```java
// doc-code: file(\"src/lib.rs\").line()[2, 5]
```
".to_string());
        let string = rmd.parse();
        assert_eq!("233333
```java
extern crate pest_derive;

mod wmd;
```", string)
    }
}
