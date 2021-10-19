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
    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I, W> TextWriter<'a, I, W>
    where
        I: Iterator<Item = Event<'a>>,
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
                    // escape_html(&mut self.writer, &text)?;
                    write!(&mut self.writer, "{}", &text)?;
                    self.end_newline = text.ends_with('\n');
                }
                Code(text) => {
                    self.write("`")?;
                    // escape_html(&mut self.writer, &text)?;
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
                    let len = self.numbers.len() + 1;
                    self.write("<sup class=\"footnote-reference\"><a href=\"#")?;
                    // escape_html(&mut self.writer, &name)?;
                    self.write("\">")?;
                    let number = *self.numbers.entry(name).or_insert(len);
                    write!(&mut self.writer, "{}", number)?;
                    self.write("</a></sup>")?;
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
                self.write("")
            }
            Tag::Heading(level) => {
                write!(&mut self.writer, "{} ", "#".repeat(level as usize).as_str())
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                self.write("<table>")
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                self.write("<thead><tr>")
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.write("<tr>")
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("<th")?;
                    }
                    TableState::Body => {
                        self.write("<td")?;
                    }
                }
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => self.write(" align=\"left\">"),
                    Some(&Alignment::Center) => self.write(" align=\"center\">"),
                    Some(&Alignment::Right) => self.write(" align=\"right\">"),
                    _ => self.write(">"),
                }
            }
            Tag::BlockQuote => {
                self.write("> ")
            }
            Tag::CodeBlock(info) => {
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang.is_empty() {
                            self.write("\n```")
                        } else {
                            write!(&mut self.writer, "```{}", lang)?;
                            self.write("\n")
                        }
                    }
                    CodeBlockKind::Indented => self.write("```"),
                }
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
            Tag::Emphasis => self.write("<em>"),
            Tag::Strong => self.write("**"),
            Tag::Strikethrough => self.write("~"),
            Tag::Link(LinkType::Email, dest, title) => {
                self.write("<a href=\"mailto:")?;
                // escape_href(&mut self.writer, &dest)?;
                write!(&mut self.writer, "{}", dest)?;
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    // escape_html(&mut self.writer, &title)?;
                }
                self.write("\">")
            }
            Tag::Link(_link_type, _dest, _title) => {
                self.write("[")
            }
            Tag::Image(_link_type, _dest, _title) => {
                self.write("![")
            }
            Tag::FootnoteDefinition(name) => {
                if self.end_newline {
                    self.write("<div class=\"footnote-definition\" id=\"")?;
                } else {
                    self.write("\n<div class=\"footnote-definition\" id=\"")?;
                }
                // escape_html(&mut self.writer, &*name)?;
                self.write("\"><sup class=\"footnote-definition-label\">")?;
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name).or_insert(len);
                write!(&mut self.writer, "{}", number)?;
                self.write("</sup>")
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
                self.write("</tbody></table>\n")?;
            }
            Tag::TableHead => {
                self.write("</tr></thead><tbody>\n")?;
                self.table_state = TableState::Body;
            }
            Tag::TableRow => {
                self.write("</tr>\n")?;
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("</th>")?;
                    }
                    TableState::Body => {
                        self.write("</td>")?;
                    }
                }
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
                self.write("\n\n")?;
            }
            Tag::List(Some(_start)) => {
                self.list_index = 0;
                self.write("\n")?;
            }
            Tag::List(None) => {
                self.write("</ul>\n")?;
            }
            Tag::Item => {
                self.write("\n")?;
            }
            Tag::Emphasis => {
                self.write("</em>")?;
            }
            Tag::Strong => {
                self.write("</strong>")?;
            }
            Tag::Strikethrough => {
                self.write("</del>")?;
            }
            Tag::Link(_link_type, dest, _title) => {
                self.write("](")?;
                write!(&mut self.writer, "{}", dest)?;
                self.write(")")?;
            }
            Tag::Image(_link_type, dest, _title) => {
                self.write("](")?;
                write!(&mut self.writer, "{}", dest)?;
                self.write(")")?;
            }
            Tag::FootnoteDefinition(_) => {
                self.write("</div>\n")?;
            }
        }
        Ok(())
    }
}

pub fn push_text<'a, I>(s: &mut String, iter: I)
    where
        I: Iterator<Item = Event<'a>>,
{
    TextWriter::new(iter, s).run().unwrap();
}

pub fn write_text<'a, I, W>(writer: W, iter: I) -> io::Result<()>
    where
        I: Iterator<Item = Event<'a>>,
        W: Write,
{
    TextWriter::new(iter, WriteWrapper(writer)).run()
}


#[cfg(test)]
mod tests {
    use std::io::Write;
    use pulldown_cmark::{Event, Options, Parser, Tag};
    use crate::md_writer;

    #[test]
    fn should_parse_line() {
        let list = "1. aa
2. blabla
";
        let parser = Parser::new_ext(list, Options::empty())
            .map(|event| match event {
                Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
                _ => event,
            })
            .filter(|event| match event {
                Event::Start(Tag::Image(..)) | Event::End(Tag::Image(..)) => false,
                _ => true,
            });

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }
}
