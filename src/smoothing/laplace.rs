use std::collections::HashMap;
use crate::utils::data_parser::DataParser;

pub struct Laplace {
    ham_tokens: HashMap<u32, usize>,
    spam_tokens: HashMap<u32, usize>,
    alpha: f64,
    vocab_size: usize,
    test_email_tokens: Vec<u32>,
}


impl Laplace {
    pub fn new(data_parser: &DataParser, alpha: f64, test_email_tokens: &Vec<u32>) -> Self {
        Self {
            ham_tokens: data_parser.ham_tokens.clone(),
            spam_tokens: data_parser.spam_tokens.clone(),
            alpha,
            vocab_size: data_parser.unique_words,
            test_email_tokens: test_email_tokens.clone(),
        }
    }
}

impl Laplace {
    /// Returns the log-probability of the test email being ham using Laplace smoothing
    pub fn log_prob_ham(&self) -> f64 {
        let total_ham_tokens: usize = self.ham_tokens.values().sum();
        let mut log_prob = 0.0;
        for token in &self.test_email_tokens {
            let count = *self.ham_tokens.get(token).unwrap_or(&0);
            let prob = self.laplace_smoothing(count as u32, total_ham_tokens, self.vocab_size, self.alpha);
            log_prob += prob.ln();
        }
        log_prob
    }

    /// Returns the log-probability of the test email being spam using Laplace smoothing
    pub fn log_prob_spam(&self) -> f64 {
        let total_spam_tokens: usize = self.spam_tokens.values().sum();
        let mut log_prob = 0.0;
        for token in &self.test_email_tokens {
            let count = *self.spam_tokens.get(token).unwrap_or(&0);
            let prob = self.laplace_smoothing(count as u32, total_spam_tokens, self.vocab_size, self.alpha);
            log_prob += prob.ln();
        }
        log_prob
    }

    // Function to apply Laplace smoothing to a set of counts
    // Like in the example, we have 2 apples and 1 banana, and 3 unique words in the vocabulary
    // We want to apply Laplace smoothing to get the probability of the word "apple" being in the class
    // Formula: P(word|class) = (count(word, class) + 1) / (count(class) + V)
    fn laplace_smoothing(&self, occurrences_of_word: u32, total_words_in_class: usize, vocab_size: usize, alpha: f64) -> f64 {
        (occurrences_of_word as f64 + alpha) / (total_words_in_class as f64 + alpha * vocab_size as f64)
    }
}
