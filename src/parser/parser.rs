use pest::iterators::Pair;
use pest::Parser;

use crate::parser::ast::{ApiNode, ApiRoot, Cataloging, ConceptSource, SourceUnit, SourceUnitPart, StructField};

#[derive(Parser)]
#[grammar = "parser/forming.pest"]
struct IdentParser;

pub fn parse(text: &str) -> SourceUnit {
    let pairs = IdentParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    let mut part = vec![];

    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                Rule::concepts_decl => {
                    part.push(SourceUnitPart::ConceptSource(parse_concept_list_decl(decl)));
                }
                Rule::concept_decl => {
                    parse_concept_decl(decl)
                }
                Rule::api_root_decl => {
                    part.push(SourceUnitPart::Api(parse_api_root_decl(decl)));
                }
                _ => {
                    println!("Rule:    {:?}", decl.as_rule());
                    println!("Span:    {:?}", decl.as_span());
                }
            }
        }
    }

    SourceUnit(part)
}

fn parse_concept_decl(decl: Pair<Rule>) {
    for concepts in decl.into_inner() {
        match concepts.as_rule() {
            Rule::comments => {
                println!("COMMENT:    {:?}", concepts.tokens());
            }
            _ => {
                println!("Rule:    {:?}", concepts.as_rule());
                println!("Span:    {:?}", concepts.as_span());
            }
        }
    }
}

fn parse_concept_list_decl(decl: Pair<Rule>) -> ConceptSource {
    let mut source = ConceptSource::new();
    for concepts in decl.into_inner() {
        match concepts.as_rule() {
            Rule::source_way => {
                source.cataloging = Cataloging::from(String::from(concepts.as_str()));
            }
            Rule::string_literal => {
                source.path = string_from_pair(concepts);
            }
            _ => {
                println!("Rule:    {:?}", concepts.as_rule());
                println!("Span:    {:?}", concepts.as_span());
            }
        }
    }

    source
}

fn parse_api_root_decl(decl: Pair<Rule>) -> ApiRoot {
    let mut root = ApiRoot::new();
    for api_root in decl.into_inner() {
        match api_root.as_rule() {
            Rule::api_ident => {
                root.name = String::from(api_root.as_str());
            }
            Rule::api_body => {
                root.apis.push(parse_api_body(api_root));
            }
            _ => {
                println!("Rule:    {:?}", api_root.as_rule());
                println!("Span:    {:?}", api_root.as_span());
            }
        }
    }

    root
}

fn parse_api_body(api_root: Pair<Rule>) -> ApiNode {
    let mut node = ApiNode::new();
    for pair in api_root.into_inner() {
        match pair.as_rule() {
            Rule::api_decl => {
                parse_api_decl(pair, &mut node);
            }
            _ => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
            }
        }
    }

    node
}

fn parse_api_decl(api_root: Pair<Rule>, node: &mut ApiNode) {
    for pair in api_root.into_inner() {
        match pair.as_rule() {
            Rule::api_in_decl => {
                for in_pair in pair.into_inner() {
                    if let Rule::struct_body = in_pair.as_rule() {
                        let body = parse_struct_body(in_pair);
                        node.api_in = body;
                    }
                }
            }
            Rule::api_out_decl => {
                for in_pair in pair.into_inner() {
                    if let Rule::struct_body = in_pair.as_rule() {
                        let body = parse_struct_body(in_pair);
                        node.api_out = body;
                    }
                }
            }
            Rule::pre_cond => {
                println!("pre_cond: {:?}", pair.as_str());
            }
            Rule::post_cond => {
                println!("post_cond: {:?}", pair.as_str());
            }
            _ => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
            }
        }
    }
}

fn parse_struct_body(body_pair: Pair<Rule>) -> Vec<StructField> {
    let mut vec = vec![];
    for pair in body_pair.into_inner() {
        match pair.as_rule() {
            Rule::struct_node => {
                vec.push(parse_struct(pair));
            }
            _ => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
            }
        }
    }
    vec
}

fn parse_struct(struct_pair: Pair<Rule>) -> StructField {
    let mut node = StructField::new();
    for pair in struct_pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                node.identifier = String::from(pair.as_str());
            }
            Rule::struct_type => {
                node.declarator = StructField::parse_type(String::from(pair.as_str()));
            }
            _ => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
            }
        }
    }

    node
}

fn string_from_pair(pair: Pair<Rule>) -> String {
    replace_string_markers(pair.as_str())
}

pub fn replace_string_markers(input: &str) -> String {
    match input.chars().next().unwrap() {
        '"' => input.replace('"', ""),
        '\'' => input.replace('\'', ""),
        '`' => input.replace('`', ""),
        _ => unreachable!("error: {:?}", input),
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::ast::{Cataloging, SourceUnitPart, TypeSpecifier};
    use crate::parser::parser::parse;

    #[test]
    fn should_parse_file() {
        let file_unit = parse("concepts => file(\"concepts.csv\")");
        match &file_unit.0[0] {
            SourceUnitPart::ConceptSource(source) => {
                assert_eq!(source.cataloging, Cataloging::File);
                assert_eq!(source.path, "concepts.csv");
            }
            _ => { assert!(false); }
        };

        let dir_unit = parse("concepts => dir(\"concepts/\")");
        match &dir_unit.0[0] {
            SourceUnitPart::ConceptSource(source) => {
                assert_eq!(source.cataloging, Cataloging::Dir);
                assert_eq!(source.path, "concepts.csv");
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn should_parse_basic_concept() {
        parse("
concept '博客' {
 -- 显示博客的相关信息
            behavior { }
            struct { }
        }");

        parse("concept  Blog {
            behavior { }
            struct { }
        }");
    }

    #[test]
    fn should_parse_concept_from_source() {
        parse("concept  Blog {
            behavior { }
            struct uml::dir('').class('Blog')
        }");

        parse("concept  Blog {
            behavior { }
            struct uml::file('').class('Blog')
        }");
    }

    #[test]
    fn should_parse_basic_contract() {
        parse("contract for Blog {
            precondition {
               '博客不为空': not empty
            }
        } ");

        parse("contract for Blog {
            precondition {
               title_not_empty: not empty
            }
        } ");
    }

    #[test]
    fn should_parse_basic_api() {
        let unit = parse("api for BlogPost {
            in { title: String, description: String }
            out { blog: Blog }
            pre_cond {
               '字符串不为空': not empty
            }
            pre_cond {
               '博客不为空': 'not empty'
            }
        } ");

        match &unit.0[0] {
            SourceUnitPart::Api(api) => {
                println!("api: {:?}", api);
                assert_eq!(api.name, "BlogPost");

                let first_api = &api.apis[0];

                assert_eq!(first_api.api_in.len(), 2);
                assert_eq!(first_api.api_in[0].identifier, "title");
                assert_eq!(first_api.api_out.len(), 1);
                assert_eq!(first_api.api_out[0].declarator, TypeSpecifier::TypeType(String::from("Blog")));
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn should_parse_basic_struct() {
        parse("concept  Blog {
            struct {
                name: String
            }
        }");

        parse("struct for Blog {
                name: String
        }");
    }
}