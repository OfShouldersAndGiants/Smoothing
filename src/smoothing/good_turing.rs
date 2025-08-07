use std::collections::HashMap;

/// Good-Turing probability for one bigram.
///
/// * `total_bigram_tokens`  = N   (total bigram *tokens* in the corpus)
/// * `bigram_count`         = r   (0, 1, 2, …)
/// * `frequency_of_frequency`  = map r ↦ N_r
/// * `unseen_bigram_types`     = N₀  (number of legal bigram *types* that never appear)
pub fn good_turing(
    total_bigram_tokens: f64,
    bigram_count: usize,
    frequency_of_frequency: &HashMap<usize, usize>,
    unseen_bigram_types: f64,
) -> f64 {
    // Frequencies for the bigram count and the bigram count + 1
    let frequency_of_current_count = *frequency_of_frequency.get(&bigram_count).unwrap_or(&0) as f64;       // N_r
    let frequency_of_next_count = *frequency_of_frequency.get(&(bigram_count + 1)).unwrap_or(&0) as f64; // N_{r+1}

    // -----------------------------------------------------------------------
    // (1) UNSEEN BIGRAMS  (r = 0)   →  (N1 / N0) / N
    // -----------------------------------------------------------------------
    if bigram_count == 0 {
        let frequency_of_count_one = *frequency_of_frequency.get(&1).unwrap_or(&0) as f64;            // N1
        if unseen_bigram_types == 0.0 || frequency_of_count_one == 0.0 || total_bigram_tokens == 0.0 {
            return 0.0;                                                 // edge-case
        }
        return (frequency_of_count_one / unseen_bigram_types) / total_bigram_tokens;
    }

    // -----------------------------------------------------------------------
    // (2) SEEN BIGRAMS  (r > 0)
    //     – if we're missing N_r or N_{r+1}, fall back to raw MLE r / N
    // -----------------------------------------------------------------------
    if frequency_of_current_count == 0.0 || frequency_of_next_count == 0.0 {
        return bigram_count as f64 / total_bigram_tokens;                          // back up case
    }

    // Good-Turing adjusted count:  r* = (r + 1) * N_{r+1} / N_r
    let good_turing_adjusted_count = (bigram_count as f64 + 1.0) * (frequency_of_next_count / frequency_of_current_count);
    good_turing_adjusted_count / total_bigram_tokens
}
