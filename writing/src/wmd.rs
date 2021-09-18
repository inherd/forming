use crate::parser;

use pulldown_cmark::{
    Event::{Code, End, Start, Text},
    Options, Parser, Tag,
};
use crate::wreader::WReader;

pub struct Wmd {
    text: String,
}

impl Wmd {
    pub fn new(text: String) -> Wmd {
        Wmd { text }
    }

    pub fn parse(&mut self) -> String {
        let parser = create_markdown_parser(&self.text);
        let mut text = "".to_string();
        let mut is_in_code = false;

        for event in parser {
            match event {
                Start(Tag::CodeBlock(_info)) => {
                    is_in_code= true;
                }
                End(Tag::CodeBlock(_info)) => {
                    is_in_code= false;
                }
                Text(body) => {
                    let str = body.to_string();

                    if is_in_code && str.starts_with("// doc-") {
                        let writing = parser::parse(str.replace("//", "").as_str());
                        let result = WReader::read_doc_code(writing.code_docs[0].clone());
                        for line in result {
                            text += &format!("{}\n", line);
                        }
                    }
                }
                Code(_inline_code) => {
                    // text += &format!("`{}`", inline_code);
                }
                _ => {
                    // event
                }
            }
        }

        return text
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
        println!("{}", string);
    }
}
