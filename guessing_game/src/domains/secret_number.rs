struct SecretNumber {
    value: i32,
}

pub fn build(value: i32) -> SecretNumber {
    SecretNumber { value }
}
