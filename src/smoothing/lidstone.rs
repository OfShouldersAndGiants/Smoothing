pub fn lidstone_smooth(bigram_count: usize, context_count: usize, vocab_size: usize, alpha: f64) -> f64 {
    (bigram_count as f64 + alpha) / (context_count as f64 + alpha * vocab_size as f64)
}
