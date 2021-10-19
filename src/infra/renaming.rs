pub struct Renaming {}

impl Renaming {
    /// for `thisShouldBeCamelCase`
    pub fn camel_case() {}

    /// for `ThisShouldBePascalCase`
    pub fn pascal_case() {}

    /// for `this_should_be_snake_case`
    pub fn snake_case() {}

    /// for `this-should-be-snake-case`
    pub fn kebab_case() {}

    /// for example `this_should_be_snake_case` will be `THIS_SHOULD_BE_PASCAL_CASE`
    pub fn upper_case() {}
}