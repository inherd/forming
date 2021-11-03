use pest::iterators::Pair;
use pest::Parser;

use crate::parser::ast::{ApiNode, ApiUnit, Cataloging, ConceptBy, ConceptUnit, Interface, Parameter, SourceUnit, SourceUnitPart, StructField, TypeSpecifier};

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
                    part.push(SourceUnitPart::ConceptBy(parse_concept_list_decl(decl)));
                }
                Rule::concept_decl => {
                    part.push(SourceUnitPart::ConceptUnit(parse_concept_decl(decl)));
                }
                Rule::api_root_decl => {
                    part.push(SourceUnitPart::ApiUnit(parse_api_root_decl(decl)));
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

fn parse_concept_decl(decl: Pair<Rule>) -> ConceptUnit {
    let mut unit = ConceptUnit::new();
    for concepts in decl.into_inner() {
        match concepts.as_rule() {
            Rule::COMMENT => {
                let comments = process_comments(concepts);
                if comments.len() > 0 {
                    unit.description = format!("{}{}\n", unit.description, comments);
                }
            }
            Rule::identifier => {
                unit.identifier = String::from(concepts.as_str());
            }
            Rule::concept_extends => {
                unit.extends = parse_concept_extends(concepts);
            }
            Rule::string_literal => {
                unit.identifier = string_from_pair(concepts);
            }
            Rule::inner_struct_decl => {
                unit.structs = parse_inner_struct_decl(concepts);
            }
            Rule::inner_behavior_decl => {
                unit.behaviors = parse_inner_behavior_decl(concepts);
            }
            Rule::comments => {}
            _ => {
                println!("Rule:    {:?}", concepts.as_rule());
                println!("Span:    {:?}", concepts.as_span());
            }
        }
    }

    unit
}

fn process_comments(concepts: Pair<Rule>) -> String {
    concepts.as_str()
        .replace("-- ", "")
        .replace("--", "")
}

fn parse_concept_extends(decl: Pair<Rule>) -> Vec<String> {
    let mut source: Vec<String> = vec![];
    for pair in decl.into_inner() {
        if let Rule::identifier = pair.as_rule() {
            source.push(String::from(pair.as_str()))
        }
    }

    source
}

fn parse_inner_behavior_decl(decl: Pair<Rule>) -> Vec<Interface> {
    let mut vec = vec![];
    for in_pair in decl.into_inner() {
        if let Rule::interface_decl = in_pair.as_rule() {
            let mut interface = Interface::new();
            for inter in in_pair.into_inner() {
                match inter.as_rule() {
                    Rule::identifier => {
                        interface.identifier = String::from(inter.as_str());
                    }
                    Rule::return_type => {
                        interface.return_type = TypeSpecifier::from(String::from(inter.as_str()));
                    }
                    Rule::params => {
                        interface.params = parse_params(inter);
                    }
                    _ => {
                        println!("Rule:    {:?}", inter.as_rule());
                        println!("Span:    {:?}", inter.as_span());
                    }
                }
            }

            vec.push(interface);
        }
    }

    vec
}

fn parse_params(decl: Pair<Rule>) -> Vec<Parameter> {
    let mut vec = vec![];
    for concepts in decl.into_inner() {
        if let Rule::parameter = concepts.as_rule() {
            let mut parameter = Parameter::new();
            for pair in concepts.into_inner() {
                match pair.as_rule() {
                    Rule::identifier => {
                        parameter.identifier = String::from(pair.as_str());
                    }
                    Rule::param_type => {
                        parameter.specifier = TypeSpecifier::from(String::from(pair.as_str()));
                    }
                    _ => {}
                }
            }
            vec.push(parameter);
        }
    }
    vec
}

fn parse_inner_struct_decl(decl: Pair<Rule>) -> Vec<StructField> {
    let mut vec = vec![];
    for in_pair in decl.into_inner() {
        if let Rule::struct_body = in_pair.as_rule() {
            vec = parse_struct_body(in_pair);
        }
    }

    vec
}

fn parse_concept_list_decl(decl: Pair<Rule>) -> ConceptBy {
    let mut source = ConceptBy::new();
    for concepts in decl.into_inner() {
        match concepts.as_rule() {
            Rule::cataloging => {
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

fn parse_api_root_decl(decl: Pair<Rule>) -> ApiUnit {
    let mut root = ApiUnit::new();
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
            _ => { println!("Rule:    {:?}", pair.as_rule()); }
        }
    }
}

fn parse_struct_body(body_pair: Pair<Rule>) -> Vec<StructField> {
    let mut vec = vec![];
    for pair in body_pair.into_inner() {
        match pair.as_rule() {
            Rule::struct_node => {
                vec.push(parse_struct_field(pair));
            }
            Rule::one_line_struct => {
                vec.append(&mut parse_one_line_struct(pair));
            }
            _ => { println!("Rule:    {:?}", pair.as_rule()); }
        }
    }
    vec
}

fn parse_one_line_struct(struct_pair: Pair<Rule>) -> Vec<StructField> {
    let mut vec = vec![];
    let mut identifiers = vec![];

    for pair in struct_pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                identifiers.push(String::from(pair.as_str()));
            }
            Rule::struct_type => {
                let type_spec = TypeSpecifier::from(String::from(pair.as_str()));
                let mut fields = identifiers.clone().into_iter()
                    .map(|ident| {
                        StructField { identifier: ident.clone(), declarator: type_spec.clone() }
                    })
                    .collect::<Vec<StructField>>();

                identifiers.clear();
                vec.append(&mut fields);
            }
            _ => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
            }
        }
    }

    vec
}

fn parse_struct_field(struct_pair: Pair<Rule>) -> StructField {
    let mut node = StructField::new();
    for pair in struct_pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                node.identifier = String::from(pair.as_str());
            }
            Rule::struct_type => {
                node.declarator = TypeSpecifier::from(String::from(pair.as_str()));
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
    fn concept_by_file() {
        let file_unit = parse("concepts => file(\"concepts.csv\")");
        match &file_unit.0[0] {
            SourceUnitPart::ConceptBy(source) => {
                assert_eq!(source.cataloging, Cataloging::File);
                assert_eq!(source.path, "concepts.csv");
            }
            _ => { assert!(false); }
        };

        let dir_unit = parse("concepts => dir(\"concepts/\")");
        match &dir_unit.0[0] {
            SourceUnitPart::ConceptBy(source) => {
                assert_eq!(source.cataloging, Cataloging::Dir);
                assert_eq!(source.path, "concepts/");
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn basic_concept() {
        let unit = parse("
        concept '博客' {
                -- 显示博客的
                -- 相关信息
            behavior {
              get_absolute_url(): String;
              validate_unique();
              publish_date_since(): datetime;
            }
            struct { title: String, description: String }
        }");

        println!("{:?}", unit);
        match &unit.0[0] {
            SourceUnitPart::ConceptUnit(unit) => {
                assert_eq!(unit.identifier, "博客");
                assert_eq!(unit.description, "显示博客的\n相关信息\n");
                assert_eq!(unit.structs.len(), 2);
                assert_eq!(unit.behaviors.len(), 3);
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn full_concept_example() {
        let unit = parse("
concept Blog(Displayable, Ownable) {
    struct {
        title, slug, description, gen_description, content, featured_image: String;
        id, user_id, site_id: Integer;
        created, updated: datetime;
    }
    behavior {
        get_absolute_url(): String;
        validate_unique();
        publish_date_since(): datetime;
        published(): Integer;
        save(blog: Blog);
        delete(id: Integer);
    }
}
");

        println!("{:?}", unit);
        match &unit.0[0] {
            SourceUnitPart::ConceptUnit(unit) => {
                assert_eq!(unit.extends.len(), 2);
                assert_eq!(unit.behaviors.len(), 6);
                assert_eq!(unit.structs.len(), 11);
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn concept_from_source() {
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
    fn basic_contract() {
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
    fn basic_api() {
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
            SourceUnitPart::ApiUnit(api) => {
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
    fn basic_struct() {
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