extern crate pest;
#[macro_use]
extern crate pest_derive;

mod wmd;
pub mod parser;
pub mod wreader;

#[cfg(test)]
mod tests {
    // doc-start: section1
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    // doc-end: section1
}
