use pest::Parser;
use pest::iterators::Pair;
use crate::ast::{CodeDoc, Writing};

#[derive(Parser)]
#[grammar = "writing.pest"]
struct WritingParser;

pub fn parse(text: &str) -> Writing {
    let pairs = WritingParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    let mut writing = Writing::new();
    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                Rule::doc_code_decl => {
                    writing.code_docs.push(parse_doc_rule(decl));
                }
                _ => {
                    println!("Rule:    {:?}", decl.as_rule());
                    println!("Span:    {:?}", decl.as_span());
                    println!("Text:    {}", decl.as_str());
                },
            }
        }
    }

    writing
}

fn parse_doc_rule(decl: Pair<Rule>) -> CodeDoc {
    let mut code_doc = CodeDoc::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::string_literal => {
                code_doc.file = replace_string_markers(pair.as_str());
            }
            Rule::start_line => {
                code_doc.start_line = pair.as_str()
                    .parse()
                    .expect("convert int error");
            }
            Rule::end_line => {
                code_doc.end_line = pair.as_str()
                    .parse()
                    .expect("convert int error");
            }
            _ => {

            }
        }
    }

    code_doc
}

pub fn replace_string_markers(input: &str) -> String {
    match input.chars().next().unwrap() {
        '"' => input.replace('"', ""),
        '\'' => input.replace('\'', ""),
        _ => unreachable!("output: {:?}", input),
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn it_works() {
        let writing = parse("doc-code: file(\"src/lib.rs\").line()[2, 5]");
        assert_eq!(writing.code_docs.len(), 1);

        let doc = writing.code_docs[0].clone();

        assert_eq!("src/lib.rs", doc.file);
        assert_eq!(2, doc.start_line);
        assert_eq!(5, doc.end_line);
    }
}

