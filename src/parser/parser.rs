use pest::iterators::Pair;
use pest::Parser;

use crate::parser::ast::{ApiNode, ApiRoot, SourceUnit, SourceUnitPart, StructField};

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
                    parse_concepts(decl)
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

fn parse_concepts(decl: Pair<Rule>) {
    for concepts in decl.into_inner() {
        match concepts.as_rule() {
            _ => {
                println!("Rule:    {:?}", concepts.as_rule());
                println!("Span:    {:?}", concepts.as_span());
            }
        }
    }
}

fn parse_api_root_decl(decl: Pair<Rule>) -> ApiRoot {
    let mut root = ApiRoot::new();
    for api_root in decl.into_inner() {
        match api_root.as_rule() {
            Rule::api_ident => {
                root.name = String::from(api_root.as_str());
            }
            Rule::api_body => {
                for pair in api_root.into_inner() {
                    match pair.as_rule() {
                        Rule::api_decl => {
                            root.apis.push(parse_api_body(pair));
                        }
                        _ => {
                            println!("Rule:    {:?}", pair.as_rule());
                            println!("Span:    {:?}", pair.as_span());
                        }
                    }
                }
            }
            _ => {
                println!("Rule:    {:?}", api_root.as_rule());
                println!("Span:    {:?}", api_root.as_span());
            }
        }
    }

    root
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

fn parse_api_body(api_root: Pair<Rule>) -> ApiNode {
    let mut node = ApiNode::new();
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

    node
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse;

    #[test]
    fn should_parse_file() {
        parse("concepts => file(\"concepts.csv\")");
    }

    #[test]
    fn should_parse_dir() {
        parse("concepts => dir(\"concepts/\")");
    }

    #[test]
    fn should_parse_basic_concept() {
        parse("
// normal quote
concept '博客' {
 --  显示博客的相关信息
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
        let unit = parse("api for /search/?q=%E5%8D%9A%E5%AE%A2&type=blog.BlogPost {
            in { title: String, description: String }
            out { blog: Blog }
            pre_cond {
               '字符串不为空': not empty
            }
            pre_cond {
               '博客不为空': 'not empty'
            }
        } ");

        assert_eq!(unit.0.len(), 1);
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