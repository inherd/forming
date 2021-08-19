use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/forming.pest"]
struct IdentParser;

pub fn parse(text: &str) {
    let pairs = IdentParser::parse(Rule::start, text).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse;

    #[test]
    fn should_parse_for_basic_csv() {
        parse("concepts => {
lang: zh,lang:en,programming,domain
blog,blog,Blog
}");
    }

    #[test]
    #[ignore]
    fn should_parse_utf_8_csv() {
        parse("concepts => {
lang: zh,lang:en,programming,domain
博客,blog,blog,Blog
}");
    }
}