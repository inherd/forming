pub trait CodegenContract {
    fn pre_condition(&self);
    fn post_condition(&self);
}

pub trait LanguageCheck {
    fn verify_keywords();
}

pub trait Lang {
    fn module_specs(&self);
    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![]
    }
    fn packages(&self);
    fn comment(&self);
}