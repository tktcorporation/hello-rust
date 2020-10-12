#[derive(PartialEq, Debug)]
pub enum ComparisonResult {
    Less,
    Over,
    Equal,
}

impl ComparisonResult {
    pub fn is_equal(&self) -> bool {
        *self == ComparisonResult::Equal
    }
}
