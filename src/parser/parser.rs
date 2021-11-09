use pest::iterators::Pair;
use pest::Parser;

use crate::parser::ast::{ApiDecl, ApiNode, Behavior, BehaviorForDecl, Cataloging,
                         ConceptByDecl, ConceptDecl, ConceptSpaceDecl, ConditionDecl, ContractDecl,
                         Diagram, DiagramGroup, DiagramItem, Expression, Parameter, SourceUnit,
                         SourceUnitPart, StructField, StructForDecl, TypeSpecifier};

#[derive(Parser)]
#[grammar = "parser/forming.pest"]
struct FormingParser;

pub fn parse(text: &str) -> SourceUnit {
    // todo: mapping to pest errors
    // let pairs = match FormingParser::parse(Rule::start, text) {
    //     Ok(p) => { Some(p) }
    //     Err(e) => {
    //         let fancy_e = e.renamed_rules(|rule| {
    //             match *rule {
    //                 _ => {
    //                     "".to_string()
    //                 }
    //             }
    //         });
    //
    //         println!("{:?}", fancy_e);
    //         None
    //     }
    // };

    let pairs = FormingParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    let mut parts = vec![];
    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                Rule::concepts_decl => {
                    parts.push(SourceUnitPart::ConceptBy(parse_concept_list_decl(decl)));
                }
                Rule::concept_decl => {
                    parts.push(SourceUnitPart::Concept(parse_concept_decl(decl)));
                }
                Rule::api_root_decl => {
                    parts.push(SourceUnitPart::ApiUnit(parse_api_root_decl(decl)));
                }
                Rule::concept_space_decl => {
                    parts.push(SourceUnitPart::ConceptSpace(parse_concept_space(decl)));
                }
                Rule::struct_for_decl => {
                    parts.push(SourceUnitPart::StructFor(parse_struct_for(decl)));
                }
                Rule::contract_for_decl => {
                    parts.push(SourceUnitPart::Contract(parse_contract_for(decl)));
                }
                Rule::behavior_for_decl => {
                    parts.push(SourceUnitPart::BehaviorFor(parse_behavior_for(decl)));
                }
                Rule::diagram_decl => {
                    parts.push(SourceUnitPart::Diagram(parse_diagram_decl(decl)));
                }
                _ => { show_rule(decl); }
            }
        }
    }

    SourceUnit(parts)
}

fn parse_diagram_decl(decl: Pair<Rule>) -> Diagram {
    let mut diagram = Diagram::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                diagram.identify = String::from(pair.as_str());
            }
            Rule::diagram_group_decl => {
                diagram.groups.push(parse_diagram_group_decl(pair));
            }
            _ => { show_rule(pair) }
        }
    }
    diagram
}

fn parse_diagram_group_decl(decl: Pair<Rule>) -> DiagramGroup {
    let mut group = DiagramGroup::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::group_ident => {
                group.identify = String::from(pair.as_str());
            }
            Rule::group_item => {
                group.items.push(parse_diagram_item(pair));
            }
            Rule::diagram_group_decl => {
                group.childrens.push(parse_diagram_group_decl(pair));
            }
            _ => { show_rule(pair) }
        }
    }
    group
}

fn parse_diagram_item(decl: Pair<Rule>) -> DiagramItem {
    let mut item = DiagramItem::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                item.identify = String::from(pair.as_str());
            }
            Rule::string_literal => {
                item.description = string_from_pair(pair);
            }
            _ => { show_rule(pair) }
        }
    }
    item
}

fn parse_concept_space(decl: Pair<Rule>) -> ConceptSpaceDecl {
    let mut space = ConceptSpaceDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                space.identifier = String::from(pair.as_str());
            }
            Rule::space_body => {
                let body = parse_space_body(pair);
                space.package = body.0;
                space.concepts = body.1;
                space.type_type = body.2;
            }
            _ => { show_rule(pair); }
        }
    }

    space
}

fn parse_space_body(decl: Pair<Rule>) -> (String, Vec<String>, String) {
    let mut package: String = String::from("");
    let mut items: Vec<String> = vec![];
    let mut type_type: String = String::from("");

    for space in decl.into_inner() {
        if let Rule::space_node = space.as_rule() {
            for pair in space.into_inner() {
                match pair.as_rule() {
                    Rule::space_package_decl => {
                        let inner = pair.into_inner();
                        package = string_from_pair(inner.peek().unwrap());
                    }
                    Rule::space_type => {
                        let inner = pair.into_inner();
                        type_type = string_from_pair(inner.peek().unwrap());
                    }
                    Rule::space_concepts_decl => {
                        for inner in pair.into_inner() {
                            match inner.as_rule() {
                                Rule::identifier => {
                                    items.push(String::from(inner.as_str()));
                                }
                                _ => { show_rule(inner); }
                            }
                        }
                    }
                    _ => { show_rule(pair); }
                }
            }
        }
    }

    (package, items, type_type)
}

fn parse_struct_for(decl: Pair<Rule>) -> StructForDecl {
    let mut unit = StructForDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::struct_body => {
                unit.declarations = parse_struct_body(pair);
            }
            Rule::identifier => {
                unit.identifier = String::from(pair.as_str());
            }
            _ => { show_rule(pair); }
        }
    }
    unit
}

fn parse_behavior_for(decl: Pair<Rule>) -> BehaviorForDecl {
    let mut behavior_for = BehaviorForDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                behavior_for.identifier = String::from(pair.as_str());
            }
            Rule::interface_decl => {
                behavior_for.behaviors.push(parse_interface(pair));
            }
            _ => { show_rule(pair); }
        }
    }
    behavior_for
}

fn parse_contract_for(decl: Pair<Rule>) -> ContractDecl {
    let mut unit = ContractDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::contract_body => {
                let mut body = parse_contract_body(pair);
                unit.pre_condition.append(&mut body.0);
                unit.post_condition.append(&mut body.1);
            }
            Rule::identifier => {
                unit.identifier = String::from(pair.as_str());
            }
            Rule::string_literal => {
                unit.identifier = string_from_pair(pair);
            }
            _ => { show_rule(pair); }
        }
    }
    unit
}

fn parse_contract_body(decl: Pair<Rule>) -> (Vec<ConditionDecl>, Vec<ConditionDecl>) {
    let mut pre_conds = vec![];
    let mut post_conds = vec![];

    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::pre_cond => {
                pre_conds.append(&mut parse_all_condition(pair));
            }
            Rule::post_cond => {
                post_conds.append(&mut parse_all_condition(pair));
            }
            _ => { show_rule(pair); }
        }
    }

    (pre_conds, post_conds)
}

fn parse_all_condition(decl: Pair<Rule>) -> Vec<ConditionDecl> {
    let mut vec = vec![];
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::condition => {
                vec.push(parse_condition(pair));
            }
            _ => { show_rule(pair); }
        }
    }

    vec
}

fn parse_condition(decl: Pair<Rule>) -> ConditionDecl {
    let mut condition = ConditionDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::cond_description => {
                condition.text = string_from_pair(pair);
            }
            Rule::cond_expr => {
                condition.expr = Expression::String(String::from(pair.as_str()));
            }
            _ => { show_rule(pair); }
        }
    }

    condition
}

fn parse_concept_decl(decl: Pair<Rule>) -> ConceptDecl {
    let mut unit = ConceptDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::COMMENT => {
                let comments = process_comments(pair);
                if comments.len() > 0 {
                    unit.description = format!("{}{}\n", unit.description, comments);
                }
            }
            Rule::identifier => {
                unit.identifier = String::from(pair.as_str());
            }
            Rule::string_literal => {
                unit.identifier = string_from_pair(pair);
            }
            Rule::extends => {
                unit.extends = parse_extends(pair);
            }
            Rule::inner_struct_decl => {
                unit.struct_fields = parse_inner_struct_decl(pair);
            }
            Rule::struct_import_decl => {
                // todo: add import struct support
            }
            Rule::inner_behavior_decl => {
                unit.behaviors = parse_inner_behavior_decl(pair);
            }
            Rule::comments => {}
            _ => { show_rule(pair); }
        }
    }

    pop_last_new_line(&mut unit);
    unit
}

fn pop_last_new_line(unit: &mut ConceptDecl) {
    unit.description.pop();
}

fn process_comments(concepts: Pair<Rule>) -> String {
    concepts.as_str()
        .replace("-- ", "")
        .replace("--", "")
}

fn parse_extends(decl: Pair<Rule>) -> Vec<String> {
    let mut source: Vec<String> = vec![];
    for pair in decl.into_inner() {
        if let Rule::identifier = pair.as_rule() {
            source.push(String::from(pair.as_str()))
        }
    }

    source
}

fn parse_inner_behavior_decl(decl: Pair<Rule>) -> Vec<Behavior> {
    let mut vec = vec![];
    for in_pair in decl.into_inner() {
        if let Rule::interface_decl = in_pair.as_rule() {
            vec.push(parse_interface(in_pair));
        }
    }

    vec
}

fn parse_interface(in_pair: Pair<Rule>) -> Behavior {
    let mut interface = Behavior::new();
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
            _ => { show_rule(inter); }
        }
    }
    interface
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
                    _ => { show_rule(pair); }
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

fn parse_concept_list_decl(decl: Pair<Rule>) -> ConceptByDecl {
    let mut source = ConceptByDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::cataloging => {
                source.cataloging = Cataloging::from(String::from(pair.as_str()));
            }
            Rule::string_literal => {
                source.path = string_from_pair(pair);
            }
            _ => { show_rule(pair); }
        }
    }

    source
}

fn parse_api_root_decl(decl: Pair<Rule>) -> ApiDecl {
    let mut root = ApiDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::api_ident => {
                root.url = String::from(pair.as_str());
            }
            Rule::identifier => {
                root.identify = String::from(pair.as_str());
            }
            Rule::api_body => {
                root.apis.push(parse_api_body(pair));
            }
            _ => { show_rule(pair); }
        }
    }

    root
}

fn parse_api_body(api_root: Pair<Rule>) -> ApiNode {
    let mut node = ApiNode::new();
    for pair in api_root.into_inner() {
        match pair.as_rule() {
            Rule::api_ident => {
                node.identify = String::from(pair.as_str());
            }
            Rule::api_decl => {
                parse_api_decl(pair, &mut node);
            }
            _ => { show_rule(pair); }
        }
    }

    node
}

fn parse_api_decl(api_root: Pair<Rule>, node: &mut ApiNode) {
    for pair in api_root.into_inner() {
        match pair.as_rule() {
            Rule::inbound => {
                for in_pair in pair.into_inner() {
                    if let Rule::struct_body = in_pair.as_rule() {
                        let body = parse_struct_body(in_pair);
                        node.inbound = body;
                    }
                }
            }
            Rule::outbound => {
                for in_pair in pair.into_inner() {
                    if let Rule::struct_body = in_pair.as_rule() {
                        let body = parse_struct_body(in_pair);
                        node.outbound = body;
                    }
                }
            }
            Rule::pre_cond => {
                node.pre_cond.append(&mut parse_all_condition(pair));
            }
            Rule::post_cond => {
                node.post_cond.append(&mut parse_all_condition(pair));
            }
            _ => { show_rule(pair); }
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
            _ => { show_rule(pair); }
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
            _ => { show_rule(pair); }
        }
    }

    vec
}

fn show_rule(pair: Pair<Rule>) {
    match pair.as_rule() {
        Rule::COMMENT => {}
        _ => {
            let rule = format!("{:?}", pair.as_rule());
            if !rule.ends_with("_text") {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
            }
        }
    }
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
            _ => { show_rule(pair); }
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

        match &unit.0[0] {
            SourceUnitPart::Concept(unit) => {
                assert_eq!(unit.identifier, "博客");
                assert_eq!(unit.description, "显示博客的\n相关信息");
                assert_eq!(unit.struct_fields.len(), 2);
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

        match &unit.0[0] {
            SourceUnitPart::Concept(unit) => {
                assert_eq!(unit.extends.len(), 2);
                assert_eq!(unit.behaviors.len(), 6);
                assert_eq!(unit.struct_fields.len(), 11);
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
    fn basic_contract_for() {
        let unit = parse("contract for Blog {
            pre_cond {
               '博客不为空': not empty,
               'title_not_empty': not empty,
               'test for string expr': 'not empty';
            }
            post_cond {
                'test for string expr': 'not empty'
            }
        } ");

        match &unit.0[0] {
            SourceUnitPart::Contract(contract) => {
                assert_eq!(contract.identifier, "Blog");
                assert_eq!(contract.pre_condition.len(), 3);
                assert_eq!(contract.post_condition.len(), 1);
            }
            _ => { assert!(false); }
        }
    }

    #[test]
    fn basic_api() {
        let unit = parse("api for BlogPost(/blog) {
            /create {
                in { title: String, description: String }
                out { blog: Blog }
                pre_cond {
                   '字符串不为空': not empty
                }
                pre_cond {
                   '博客不为空': 'not empty'
                }
            }
        } ");

        match &unit.0[0] {
            SourceUnitPart::ApiUnit(api) => {
                assert_eq!(api.identify, "BlogPost");

                let first_api = &api.apis[0];

                assert_eq!(first_api.identify, "/create");
                assert_eq!(first_api.inbound.len(), 2);
                assert_eq!(first_api.inbound[0].identifier, "title");
                assert_eq!(first_api.outbound.len(), 1);
                assert_eq!(first_api.outbound[0].declarator, TypeSpecifier::TypeType(String::from("Blog")));
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn struct_for_one() {
        let unit = parse("struct for Blog {
    name: String,
    title: String
}");

        match &unit.0[0] {
            SourceUnitPart::StructFor(node) => {
                assert_eq!(node.identifier, "Blog");
                assert_eq!(node.declarations.len(), 2);

                let field = &node.declarations[0];

                assert_eq!(field.identifier, "name");
                assert_eq!(field.declarator, TypeSpecifier::String);
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn concept_space() {
        let unit = parse("space Blog {
   package: 'com.phodal.blog', // or path
   type: 'Entity',
   items: { Blog, BlogCategory, BlogCategories, BlogRelatedPosts, Comments }
}
");

        match &unit.0[0] {
            SourceUnitPart::ConceptSpace(node) => {
                assert_eq!(node.identifier, "Blog");
                assert_eq!(node.package, "com.phodal.blog");
                assert_eq!(node.type_type, "Entity");
                assert_eq!(node.concepts.len(), 5);
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn concept_space_not_package() {
        let unit = parse("space Blog {
   items: { Blog, BlogCategory, BlogCategories, BlogRelatedPosts, Comments }
}
");

        match &unit.0[0] {
            SourceUnitPart::ConceptSpace(node) => {
                assert_eq!(node.identifier, "Blog");
                assert_eq!(node.package, "");
                assert_eq!(node.concepts.len(), 5);
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn behavior_for() {
        let unit = parse("behavior for Blog {
  delete(id: Integer);
}");

        match &unit.0[0] {
            SourceUnitPart::BehaviorFor(node) => {
                assert_eq!(node.identifier, "Blog");
                assert_eq!(node.behaviors.len(), 1);
            }
            _ => { assert!(false); }
        };
    }

    #[test]
    fn basic_diagram() {
        let unit = parse("diagram Sample {
  group office {
    bank('手机银行');
    internet_bank('网银');
  }
  group backend {
    paas;
    iaas;
    group bff {
        something;
    }
  }
}");

        match &unit.0[0] {
            SourceUnitPart::Diagram(node) => {
                assert_eq!("Sample", node.identify);
                assert_eq!(2, node.groups.len());
                let first = &node.groups[0];
                assert_eq!("office", first.identify);

                assert_eq!("bank", first.items[0].identify);
                assert_eq!("手机银行", first.items[0].description);

                assert_eq!("internet_bank", first.items[1].identify);
                assert_eq!("网银", first.items[1].description);

                assert_eq!(1, node.groups[1].childrens.len())
            }
            _ => { assert!(false); }
        };
    }
}