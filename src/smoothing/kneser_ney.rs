/*
Kneser-Ney Bigram Formula:

    P_KN(w₂ | w₁) =
        max(c(w₁, w₂) - D, 0) / c(w₁)
        + λ(w₁) × P_cont(w₂)

Where:
    - c(w₁, w₂): Count of the bigram (w₁, w₂)
    - c(w₁): Total tokens that follow w₁ (bigram denominator)
    - D: Absolute discount (conventionally 0.75 on tiny corpora)
    - N₁⁺(w₁∙): Number of unique followers of w₁
    - N₁⁺(∙w₂): Number of unique predecessors of w₂
    - N₁⁺(∙∙): Total number of distinct bigram types
    - λ(w₁): Back-off weight for w₁ = D * N₁⁺(w₁∙) / c(w₁)
    - P_cont(w₂): Continuation probability of w₂ = N₁⁺(∙w₂) / N₁⁺(∙∙)
*/

pub fn kneser_ney(
    count_of_this_bigram: usize,            // c(w1,w2)
    bigrams_that_start_with_w1: usize,      // c(w1)
    unique_followers_of_w1: usize,          // N1+(w1·)
    unique_predecessors_of_w2: usize,       // N1+(·w2)
    total_distinct_bigram_types: usize,     // N1+(··)
    absolute_discount: f64,                 // D
) -> f64 {
    assert!(total_distinct_bigram_types > 0, "empty model");

    let p_cont = unique_predecessors_of_w2 as f64 / total_distinct_bigram_types as f64;

    // If the context w1 never appears with a follower, back off fully.
    if bigrams_that_start_with_w1 == 0 {
        return p_cont;                      // λ(w1)=1, direct term = 0
    }

    let direct = ((count_of_this_bigram as f64 - absolute_discount).max(0.0))
        / bigrams_that_start_with_w1 as f64;

    let lambda = absolute_discount * unique_followers_of_w1 as f64
        / bigrams_that_start_with_w1 as f64;

    direct + lambda * p_cont
}
