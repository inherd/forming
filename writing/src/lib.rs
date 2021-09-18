extern crate pest;
#[macro_use]
extern crate pest_derive;

mod wmd;
mod parser;
mod ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
