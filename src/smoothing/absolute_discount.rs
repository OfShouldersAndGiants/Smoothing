/// Absolute discount smoothing for bigram probabilities.
///
/// Formula: P = max(c − D, 0)/c(h) + D·T(h)/c(h) × P_backoff(w).
/// c = count(h,w), c(h) = context count, T(h) = distinct continuations.
///
/// Given the corpus:
///    ["the cat sat on the mat",
///     "the dog sat on the log",
///     "the cat chased the dog",
///     "the dog ran after the mouse"]
///
/// For ("the","cat"): c=2, c(h)=8, T(h)=5, c(w)=2, N=24, D=0.75.
/// discounted = 0.15625, λ = 0.46875, back-off = 0.0833.
/// Final P = 0.15625 + 0.46875 × 0.0833 ≈ 0.195
pub fn absolute_discount_smooth(
    bigram_count: usize,             // count of the bigram ("the", "cat") it is the "c" in the formula
    context_count: usize,            // count of the context ("the") it is the "c(h)" in the formula
    num_unique_continuations: usize, // number of unique words following "the" it is the "T(h)" in the formula
    unigram_count_w: usize,          // count of the word ("cat") it is the "c(w)" in the formula
    total_unigram_tokens: usize,     // total number of tokens in the corpusf
    discount: f64,                   // discount value (D)
) -> f64 {
    // println!("bigram_count: {}", bigram_count);
    // println!("context_count: {}", context_count);
    // println!("num_unique_continuations: {}", num_unique_continuations);
    // println!("unigram_count_w: {}", unigram_count_w);
    // println!("total_unigram_tokens: {}", total_unigram_tokens);
    // println!("discount: {}", discount);

    // Context never appears, or no unique continuations. We use unigram probability, (backoff) in order to avoid division by zero.
    if context_count == 0 || num_unique_continuations == 0 {
        return unigram_count_w as f64 / total_unigram_tokens as f64;
    }
    let discounted = ((bigram_count as f64 - discount).max(0.0)) / context_count as f64;
    let lambda = (discount * num_unique_continuations as f64) / context_count as f64;
    let backoff_prob = unigram_count_w as f64 / total_unigram_tokens as f64;
    discounted + lambda * backoff_prob
}
