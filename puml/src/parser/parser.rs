use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/puml.pest"]
struct PumlParser;

fn parse(text: &str) {
    let pairs = PumlParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                _ => {
                    println!("Rule:    {:?}", decl.as_rule());
                    println!("Span:    {:?}", decl.as_span());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse;

    #[test]
    fn concept_by_file() {
        parse("@startuml
/'
many lines comments
here
'/
@enduml");
    }
}