use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "parser/forming.pest"]
struct IdentParser;

pub fn parse(text: &str) {
    let pairs = IdentParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                Rule::concepts => {
                    parse_concepts(decl)
                }
                _ => {
                    println!("Rule:    {:?}", decl.as_rule());
                    println!("Span:    {:?}", decl.as_span());
                    println!("Text:    {}", decl.as_str());
                },
            }
        }
    }
}

fn parse_concepts(decl: Pair<Rule>) {
    for concepts in decl.into_inner() {
        match concepts.as_rule() {
            Rule::csv => {
                for field in concepts.into_inner() {
                    match field.as_rule() {
                        Rule::text_field => {
                            println!("Rule:    {:?}", field.as_rule());
                            println!("Span:    {:?}", field.as_span());
                        }
                        Rule::string_field => {
                            println!("Rule:    {:?}", field.as_rule());
                            println!("Span:    {:?}", field.as_span());
                        }
                        _ => {
                            println!("Rule:    {:?}", field.as_rule());
                            println!("Span:    {:?}", field.as_span());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse;

    #[test]
    fn should_parse_for_basic_csv() {
        parse("concepts => {
lang: zh,lang:en,programming,domain
blog, blog, Blog
}");
    }

    #[test]
    fn should_parse_utf_8_csv() {
        parse("concepts => {
'lang: zh','lang:en', 'programming', 'domain'
'博客', 'blog', 'blog', 'Blog'
}");
    }
}