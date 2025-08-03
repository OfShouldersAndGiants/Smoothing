use std::collections::HashMap;

pub fn good_turing(bigrams_count: f64, bigram_count: f64, frequencies: &HashMap<usize, usize>) -> f64 {

    // We need to get the frequency for the bigram count and for the bigram count + 1
    let frequency = (frequencies.get(&(bigram_count as usize)).unwrap_or(&0).clone()) as f64;
    let frequency_plus_one = (frequencies.get(&((bigram_count + 1.0) as usize)).unwrap_or(&0).clone()) as f64;

    // Handle the case where bigram_count is 0 (unseen bigrams)
    if bigram_count == 0.0 {
        // For unseen bigrams, use frequency of 1 (N1) divided by total bigrams
        let probability: f64 = (frequency) / (bigrams_count);
        return probability;
    }

    // Get the adjusted count
    let adjusted_count: f64 = (bigram_count + 1.0) * (frequency_plus_one) / frequency;

    // Get probability
    let probability: f64 = (adjusted_count) / (bigrams_count);

    return probability
}
