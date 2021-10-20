// Copyright 2015 Google Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! HTML renderer that takes an iterator of events as input.

use std::collections::HashMap;
use std::io::{self, Write};

use pulldown_cmark::escape::{StrWrite, WriteWrapper};
use pulldown_cmark::Event::*;
use pulldown_cmark::{Alignment, CodeBlockKind, CowStr, Event, LinkType, Tag};
use crate::parser::parser;
use crate::wreader::WReader;

enum TableState {
    Head,
    Body,
}

struct TextWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether or not the last write wrote a newline.
    end_newline: bool,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    list_index: usize,
    #[allow(dead_code)]
    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I, W> TextWriter<'a, I, W>
    where
        I: Iterator<Item=Event<'a>>,
        W: StrWrite,
{
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            list_index: 0,
            numbers: HashMap::new(),
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_str(s)?;

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    fn run(mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    write!(&mut self.writer, "{}", &text)?;
                    self.end_newline = text.ends_with('\n');
                }
                Code(text) => {
                    self.write("`")?;
                    write!(&mut self.writer, "{}", &text)?;
                    self.write("`")?;
                }
                Html(html) => {
                    self.write(&html)?;
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("\n")?;
                }
                Rule => {
                    if self.end_newline {
                        self.write("---\n")?;
                    } else {
                        self.write("\n---\n")?;
                    }
                }
                FootnoteReference(name) => {
                    write!(&mut self.writer, "[^{}]", name)?;
                }
                TaskListMarker(true) => {
                    self.write("[x]")?;
                }
                TaskListMarker(false) => {
                    self.write("[ ]")?;
                }
            }
        }
        Ok(())
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                Ok(())
            }
            Tag::Heading(level) => {
                write!(&mut self.writer, "{} ", "#".repeat(level as usize).as_str())
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                Ok(())
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                Ok(())
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                Ok(())
            }
            Tag::TableCell => {
                self.write("|")?
            }
            Tag::BlockQuote => {
                self.write("> ")
            }
            Tag::CodeBlock(info) => {
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang.is_empty() {
                            return self.write("\n```");
                        } else {
                            write!(&mut self.writer, "```{}", lang)?;
                            self.write("\n")?;
                        }
                    }
                    CodeBlockKind::Indented => {
                        return self.write("```");
                    }
                }

                let mut code_text = String::new();
                if let Some(Event::Text(s)) = self.iter.next() {
                    code_text.push_str(&s);
                }

                for line in code_text.lines() {
                    if line.starts_with("// doc-") {
                        let writing = parser::parse(line.replace("//", "").as_str());

                        for code in WReader::read_doc_code(writing.code_docs[0].clone()) {
                            write!(&mut self.writer, "{}\n", code)?;
                        }
                    } else {
                        write!(&mut self.writer, "{}\n", line)?;
                    }
                }


                Ok(())
            }
            Tag::List(Some(1)) => {
                self.list_index = 1;
                Ok(())
            }
            Tag::List(Some(start)) => {
                self.list_index = start as usize;
                Ok(())
            }
            Tag::List(None) => {
                self.write("- \n")
            }
            Tag::Item => {
                if self.list_index > 0 {
                    write!(&mut self.writer, "{}. ", self.list_index)?;
                    self.list_index = self.list_index + 1;
                    Ok(())
                } else {
                    self.write("")
                }
            }
            Tag::Emphasis => self.write("*"),
            Tag::Strong => self.write("**"),
            Tag::Strikethrough => self.write("~~"),
            Tag::Link(LinkType::Email, _dest, _title) => {
                self.write("<")
            }
            Tag::Link(_link_type, _dest, _title) => {
                self.write("[")
            }
            Tag::Image(_link_type, _dest, _title) => {
                self.write("![")
            }
            Tag::FootnoteDefinition(name) => {
                write!(&mut self.writer, "[^{}]: ", &*name)
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                self.write("\n\n")?;
            }
            Tag::Heading(_level) => {
                self.write("\n\n")?;
            }
            Tag::Table(_) => {
                self.write("\n")?;
            }
            Tag::TableHead => {
                self.write("|\n")?;
                self.table_state = TableState::Body;

                self.write("|")?;
                write!(&mut self.writer, "{} ", "------|".repeat(self.table_alignments.len()).as_str())?;
                self.write("\n")?;
            }
            Tag::TableRow => {
                self.write("|\n")?;
            }
            Tag::TableCell => {
                self.table_cell_index += 1;
            }
            Tag::BlockQuote => {
                self.write("")?;
            }
            Tag::CodeBlock(_) => {
                self.write("```\n\n")?;
            }
            Tag::List(Some(1)) => {
                self.list_index = 0;
                self.write("\n")?;
            }
            Tag::List(Some(_start)) => {
                self.list_index = 0;
                self.write("\n")?;
            }
            Tag::List(None) => {
                self.write("\n\n")?;
            }
            Tag::Item => {
                self.write("\n")?;
            }
            Tag::Emphasis => {
                self.write("*")?;
            }
            Tag::Strong => {
                self.write("**")?;
            }
            Tag::Strikethrough => {
                self.write("~~")?;
            }
            Tag::Link(link_type, dest, _title) => {
                match link_type {
                    LinkType::Email => {
                        self.write(">")?;
                    }
                    _ => {
                        self.write("](")?;
                        write!(&mut self.writer, "{}", dest)?;
                        self.write(")")?;
                    }
                }
            }
            Tag::Image(_link_type, dest, _title) => {
                self.write("](")?;
                write!(&mut self.writer, "{}", dest)?;
                self.write(")")?;
            }
            Tag::FootnoteDefinition(_) => {
                self.write("\n")?;
            }
        }
        Ok(())
    }
}

pub fn push_text<'a, I>(s: &mut String, iter: I)
    where
        I: Iterator<Item=Event<'a>>,
{
    TextWriter::new(iter, s).run().unwrap();
}

pub fn write_text<'a, I, W>(writer: W, iter: I) -> io::Result<()>
    where
        I: Iterator<Item=Event<'a>>,
        W: Write,
{
    TextWriter::new(iter, WriteWrapper(writer)).run()
}


#[cfg(test)]
mod tests {
    use std::io::Write;
    use pulldown_cmark::{Options, Parser};
    use crate::md_writer;

    #[test]
    fn should_parse_list() {
        let list = "1. normal
2. **strong**
3. ~~delete~~
4. *Italic*
5. ***BoldAndItalic***
";
        let parser = Parser::new(list);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }

    #[test]
    fn should_parse_footnote() {
        let list = "footnote[^1]

[^1]: My reference.
";
        let parser = Parser::new_ext(list, Options::ENABLE_FOOTNOTES);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }

    #[test]
    fn should_build_email() {
        let list = "<fake@example.com>
";
        let parser = Parser::new_ext(list, Options::ENABLE_TASKLISTS);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }

    #[test]
    fn should_build_table() {
        let list = "
| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |
";
        let parser = Parser::new_ext(list, Options::ENABLE_TABLES);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }
}
