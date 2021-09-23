use crate::parser;

use pulldown_cmark::{Options, Parser, Tag, CodeBlockKind, Event};
use crate::wreader::WReader;
use std::borrow::Cow;

pub struct Wmd {
    text: String,
}

enum Line<'a> {
    Hidden(&'a str),
    Shown(Cow<'a, str>),
}

impl<'a> Line<'a> {
    fn for_code(self) -> Cow<'a, str> {
        match self {
            Line::Shown(l) => l,
            Line::Hidden(l) => Cow::Borrowed(l),
        }
    }
}

fn map_line(s: &str) -> Line<'_> {
    let trimmed = s.trim();
    if trimmed.starts_with("##") {
        Line::Shown(Cow::Owned(s.replacen("##", "#", 1)))
    } else if let Some(stripped) = trimmed.strip_prefix("# ") {
        // # text
        Line::Hidden(&stripped)
    } else if trimmed == "#" {
        // We cannot handle '#text' because it could be #[attr].
        Line::Hidden("")
    } else {
        Line::Shown(Cow::Borrowed(s))
    }
}


impl Wmd {
    pub fn new(text: String) -> Wmd {
        Wmd { text }
    }

    pub fn parse(&mut self) -> String {
        let doc = &self.text;
        let mut parser = create_markdown_parser(doc).into_offset_iter();
        let mut text = "".to_string();
        let mut lang_code = "".to_string();

        while let Some((event, _offset)) = parser.next() {
            match event {
                Event::Start(Tag::CodeBlock(info)) => {
                    match info {
                        CodeBlockKind::Fenced(lang) => {
                            lang_code = format!("{}", lang);
                        }
                        CodeBlockKind::Indented => {}
                    }

                    let mut test_s = String::new();

                    while let Some((Event::Text(s), _)) = parser.next() {
                        test_s.push_str(&s);
                    }
                    let str = test_s
                        .lines()
                        .map(|l| map_line(l).for_code())
                        .collect::<Vec<Cow<'_, str>>>()
                        .join("\n");

                    if str.starts_with("// doc-") {
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
                Event::Text(body) => {
                    let str = body.to_string();
                    text += &format!("{}\n", str);
                }
                Event::Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                }
                Event::Html(html) => {
                    text += &format!("{}\n", html);
                }
                Event::FootnoteReference(footnote) => {
                    text += &format!("{}\n", footnote);
                }
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
