pub fn laplace_smooth(bigram_count: usize, context_count: usize, vocab_size: usize) -> f64 {
    (bigram_count as f64 + 1.0) / (context_count as f64 + vocab_size as f64)
}
