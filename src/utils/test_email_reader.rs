use std::fs;
use crate::utils::tokenizer::create_string_tokens;

pub fn read_test_email(email_path: &str) -> Vec<u32> {
    let email = fs::read_to_string(email_path)
        .expect("Failed to read email file");
    create_string_tokens(&email)
}
