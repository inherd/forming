use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "writing.pest"]
struct WritingParser;

pub fn parse(text: &str) {
    let pairs = WritingParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for decl in pair.into_inner() {
            match decl.as_rule() {
                _ => {
                    println!("Rule:    {:?}", decl.as_rule());
                    println!("Span:    {:?}", decl.as_span());
                    println!("Text:    {}", decl.as_str());
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn it_works() {
        parse("doc-code: file(\"src/lib.rs\").line()[2, 5]")
    }
}

