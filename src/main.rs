use smoothing::{laplace, lidstone};
use std::collections::HashMap;
mod smoothing;

struct Corpus {
    // Unigram counts: To know how often each word appears in the corpus
    // This helps us calculate the denominator for bigram probabilities
    unigram_counts: HashMap<String, usize>,
    // Bigram counts: To know how often each word pair appears in the corpus
    // This helps us estimate P(word2 | word1) = count(word1,word2) / count(word1)
    bigram_counts: HashMap<(String, String), usize>,
    // Vocabulary: The set of unique words in the corpus
    // This is crucial for smoothing methods like Laplace and Lidstone
    // which need to know the vocabulary size to handle unseen words
    vocab: std::collections::HashSet<String>,
    // Test bigrams: A set of bigrams to test smoothing methods on
    // Includes seen, unseen but plausible, and completely unseen bigrams
    test_bigrams: Vec<(String, String)>,
}

impl Corpus {
    /// Initializes the Corpus struct, parses the sentences, and fills in all counts and vocab.
    fn new() -> Self {
        // Define the corpus as a vector of sentences
        let sentences = vec![
            "the cat sat on the mat",
            "the dog sat on the log",
            "the cat chased the dog",
            "the dog ran after the mouse",
        ];
        let mut unigram_counts = HashMap::new();
        let mut bigram_counts = HashMap::new();
        let mut vocab = std::collections::HashSet::new();
        // Parse each sentence to extract unigrams, bigrams, and vocabulary
        for sentence in &sentences {
            let tokens: Vec<&str> = sentence.split_whitespace().collect();
            for i in 0..tokens.len() {
                // Collecting unigram counts
                *unigram_counts.entry(tokens[i].to_string()).or_insert(0) += 1;
                // Collecting vocabulary
                vocab.insert(tokens[i].to_string());
                // Collecting bigram counts
                if i + 1 < tokens.len() {
                    *bigram_counts
                        .entry((tokens[i].to_string(), tokens[i + 1].to_string()))
                        .or_insert(0) += 1;
                }
            }
        }
        // Define test bigrams with real words
        // We create a set of test bigrams that includes:
        // - Seen bigrams: Pairs that appear in our training corpus
        // - Unseen but plausible bigrams: Pairs that don't appear but use known words
        // - Completely unseen bigrams: Pairs that use words not in our vocabulary
        // This variety helps us test how well our smoothing methods handle different scenarios
        let test_bigrams = vec![
            ("the".to_string(), "cat".to_string()),   // seen
            ("cat".to_string(), "sat".to_string()),   // seen
            ("dog".to_string(), "sat".to_string()),   // seen
            ("the".to_string(), "dog".to_string()),   // seen
            ("cat".to_string(), "dog".to_string()),   // seen
            ("dog".to_string(), "mat".to_string()),   // unseen but plausible
            ("mat".to_string(), "the".to_string()),   // unseen
            ("mouse".to_string(), "ran".to_string()), // completely unseen
        ];
        Corpus {
            unigram_counts,
            bigram_counts,
            vocab,
            test_bigrams,
        }
    }

    /// Returns the count of a unigram (word) in the corpus
    fn get_unigram_count(&self, word: &str) -> usize {
        self.unigram_counts.get(word).cloned().unwrap_or(0)
    }

    /// Returns the count of a bigram (word pair) in the corpus
    fn get_bigram_count(&self, bigram: &(String, String)) -> usize {
        self.bigram_counts.get(bigram).cloned().unwrap_or(0)
    }

    /// Returns the size of the vocabulary (number of unique words)
    fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Returns a reference to the test bigrams
    fn test_bigrams(&self) -> &Vec<(String, String)> {
        &self.test_bigrams
    }
}

fn main() {
    // 1. The first thing we do is define our corpus using the Corpus struct
    //    All parsing, counting, and test bigram logic is encapsulated in Corpus
    let corpus = Corpus::new();

    // 2. Get the vocabulary size for use in smoothing methods
    let vocab_size = corpus.vocab_size();

    // 3. For each test bigram, apply smoothing methods
    //    We print the results for Laplace and Lidstone smoothing
    println!("Bigram\t\tLaplace\tLidstone");

    for bigram in corpus.test_bigrams() {
        // Set up our variables for the smoothing methods
        let context = &bigram.0;                                    // The first word in the bigram
        let word = &bigram.1;                                       // The second word in the bigram
        let context_count = corpus.get_unigram_count(context);      // How many times the context appears
        let bigram_count = corpus.get_bigram_count(bigram);         // How many times the bigram appears


        // Laplace smoothing is a simple method that adds 1 to all counts and divides by (context_count + vocab_size)
        let laplace = laplace::laplace_smooth(bigram_count, context_count, vocab_size);
        // Lidstone smoothing is a more flexible method that uses a parameter λ (here we use 0.5)
        let lidstone = lidstone::lidstone_smooth(bigram_count, context_count, vocab_size, 0.5);


        // A simple visualization of the results ;)
        println!(
            "{:<20} {:<8.4} {:<8.4}",
            format!("({:?}, {:?})", context, word),
            laplace,
            lidstone
        );
    }
}
