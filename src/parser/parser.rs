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
            Rule::string => {
                println!("Rule:    {:?}", concepts.as_rule());
                println!("Span:    {:?}", concepts.as_span());
            }
            _ => {}
        }
    }
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
}