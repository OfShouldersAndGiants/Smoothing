use std::collections::HashMap;
mod smoothing;

fn main() {
    // 1. The first thing we do is define our corpus, we are going to use a vector of strings
    //    Each string is a sentence, and each sentence is a list of words. We are using a simple corpus
    //    because it's small enough to demonstrate the concepts while still containing enough variety
    //    to show different smoothing scenarios (seen and unseen bigrams)
    let corpus = vec![
        "the cat sat on the mat",
        "the dog sat on the log",
        "the cat chased the dog",
        "the dog ran after the mouse",
    ];

    // 2. Extract bigrams and unigrams
    // We need to track three different things for language modeling:
    // 1. Bigram counts: To know how often each word pair appears in the corpus
    //    This helps us estimate P(word2 | word1) = count(word1,word2) / count(word1)
    // 2. Unigram counts: To know how often each word appears in the corpus
    //    This helps us calculate the denominator for bigram probabilities
    // 3. Vocabulary: To know the total number of unique words in the corpus
    //    This is crucial for smoothing methods like Laplace and Lidstone
    //    which need to know the vocabulary size to handle unseen words

    // Bigrams: [("the", "cat"), ("cat", "sat"), ("sat", "on")...]
    let mut bigram_counts: HashMap<(String, String), usize> = HashMap::new();
    // Unigrams: [("the", 3), ("cat", 2), ("sat", 1)...]
    let mut unigram_counts: HashMap<String, usize> = HashMap::new();
    // Vocab: ["the", "cat", "sat", "on", "the", "mat", "dog", "ran", ...]
    let mut vocab: std::collections::HashSet<String> = std::collections::HashSet::new();

    for sentence in &corpus {
        let tokens: Vec<&str> = sentence.split_whitespace().collect();
        for i in 0..tokens.len() {
            // Collecting unigram counts
            *unigram_counts.entry(tokens[i].to_string()).or_insert(0) += 1;
            // Collecting vocabulary
            vocab.insert(tokens[i].to_string());
            // Collecting bigram counts
            if i + 1 < tokens.len() {
                *bigram_counts.entry((tokens[i].to_string(), tokens[i + 1].to_string())).or_insert(0) += 1;
            }
        }
    }
    // We need to know the size of our vocabulary to use in our smoothing methods
    // This is the total number of unique words in the corpus
    let vocab_size = vocab.len();

    // 3. Define test bigrams with real words
    // We create a set of test bigrams that includes:
    // - Seen bigrams: Pairs that appear in our training corpus
    // - Unseen but plausible bigrams: Pairs that don't appear but use known words
    // - Completely unseen bigrams: Pairs that use words not in our vocabulary
    // This variety helps us test how well our smoothing methods handle different scenarios
    let test_bigrams = vec![
        ("the".to_string(), "cat".to_string()),      // seen
        ("cat".to_string(), "sat".to_string()),      // seen
        ("dog".to_string(), "sat".to_string()),      // seen
        ("the".to_string(), "dog".to_string()),      // seen
        ("cat".to_string(), "dog".to_string()),      // seen
        ("dog".to_string(), "mat".to_string()),      // unseen but plausible
        ("mat".to_string(), "the".to_string()),      // unseen
        ("mouse".to_string(), "ran".to_string()),    // completely unseen
    ];

    // 4. For each test bigram, apply smoothing methods

    println!("Bigram\t\tLaplace\tLidstone");
    for bigram in &test_bigrams {
        // Now we are going to set up our variables for the smoothing methods
        let context = &bigram.0; // This is the first word in the bigram. In ("the", "cat") context is "the"
        let word = &bigram.1;    // This is the second word in the bigram. In ("the", "cat") word is "cat"
        let context_count = unigram_counts.get(context).cloned().unwrap_or(0); // How many times "the" appears in the corpus
        let bigram_count = bigram_counts.get(bigram).cloned().unwrap_or(0);    // How many times ("the", "cat") appears in the corpus

        // These are the smoothing methods we are going to use
        // Laplace smoothing is a simple method that adds 1 to all counts and divides by (context_count + vocab_size)
        // Lidstone smoothing is a more flexible method that uses a parameter λ (we are going to use 0.5)
        let laplace = smoothing::laplace::laplace_smooth(bigram_count, context_count, vocab_size);
        let lidstone = smoothing::lidstone::lidstone_smooth(bigram_count, context_count, vocab_size, 0.5);

        // A simple visualization of the results ;)
        println!(
            "{:<20} {:<8.4} {:<8.4}",
            format!("({:?}, {:?})", context, word),
            laplace,
            lidstone
        );
    }
}

