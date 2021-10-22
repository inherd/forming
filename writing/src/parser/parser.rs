use pest::iterators::Pair;
use pest::Parser;

use crate::parser::ast::{CodeBlock, CodeDep, CodeFunc, CodeSection, CodeSource, Writing};

#[derive(Parser)]
#[grammar = "parser/writing.pest"]
struct WritingParser;

pub fn parse(text: &str) -> Writing {
    let pairs = WritingParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    let mut writing = Writing::new();
    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                Rule::doc_code_decl => {
                    writing.code_docs.push(parse_doc_decl(decl));
                }
                Rule::code_dep_decl => {
                    writing.code_deps.push(parse_deps_decl(decl));
                }
                Rule::code_section_decl => {
                    writing.code_sections.push(parse_code_sections(decl));
                }
                Rule::code_func_decl => {
                    writing.code_funcs.push(parse_code_func(decl));
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

fn parse_deps_decl(decl: Pair<Rule>) -> CodeDep {
    let mut code_dep = CodeDep::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::artifact_id => {
                code_dep.artifact_id = String::from(pair.as_str());
            }
            Rule::group_id => {
                code_dep.group_id = String::from(pair.as_str());
            }
            Rule::version => {
                code_dep.version = String::from(pair.as_str());
            }
            _ => {

            }
        }
    }
    code_dep
}

fn parse_code_sections(decl: Pair<Rule>) -> CodeSection {
    let mut section = CodeSection::new();
    let mut block = CodeBlock::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::string_literal => {
                block.file = parse_string(pair.as_str());
            }
            Rule::section_name => {
                block.name = parse_string(pair.as_str());
            }
            _ => {

            }
        }
    }
    section.blocks.push(block);
    section
}


fn parse_code_func(decl: Pair<Rule>) -> CodeFunc {
    let mut func = CodeFunc::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::string_literal => {
                func.file = parse_string(pair.as_str());
            }
            Rule::func_name => {
                func.funcs.push(parse_string(pair.as_str()))
            }
            _ => {}
        }
    }
    func
}

fn parse_doc_decl(decl: Pair<Rule>) -> CodeSource {
    let mut code_doc = CodeSource::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::string_literal => {
                code_doc.file = parse_string(pair.as_str());
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

pub fn parse_string(input: &str) -> String {
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
    fn should_parse_doc_code() {
        let writing = parse("doc-code: file(\"src/lib.rs\").line()[2, 5]");
        assert_eq!(writing.code_docs.len(), 1);

        let doc = writing.code_docs[0].clone();

        assert_eq!("src/lib.rs", doc.file);
        assert_eq!(2, doc.start_line);
        assert_eq!(5, doc.end_line);
    }

    #[test]
    fn should_parse_section() {
        let writing = parse("doc-section: file(\"src/lib.rs\").section(\"section1\")");
        assert_eq!(writing.code_sections.len(), 1);
    }

    #[test]
    fn should_parse_doc_dep() {
        let writing = parse("code-dep: colored;version=1.8.0");
        assert_eq!(writing.code_deps.len(), 1);
        let dep = &writing.code_deps[0];

        assert_eq!("1.8.0", dep.version);
        assert_eq!("colored", dep.artifact_id);
    }

    #[test]
    fn should_parse_function() {
        let writing = parse("doc-func: file(\"src/lib.rs\").func()[\"it_works\"]");
        assert_eq!(writing.code_funcs.len(), 1);
        assert_eq!(writing.code_funcs[0].file, "src/lib.rs");

        let writing = parse("doc-func: file(\"src/lib.rs\").func()[\"it_works\", \"should_parse_function\"]");
        assert_eq!(writing.code_funcs[0].funcs.len(), 2);
    }
}

