use tiktoken_rs::r50k_base;

pub fn create_string_tokens(email: &str) -> Vec<u32> {
    let bpe = r50k_base().unwrap();
    let mut word_tokens = Vec::new();
    for word in email.split_whitespace() {
        let mut tokens = bpe.encode_with_special_tokens(word);
        word_tokens.append(&mut tokens);
    }
    word_tokens
}
