use std::collections::HashMap;

pub struct Corpus {
    // Bigram counts: To know how often each word pair appears in the corpus
    // This helps us estimate P(word2 | word1) = count(word1,word2) / count(word1)
    bigram_counts: HashMap<(String, String), usize>,
    // Unigram counts: To know how often each word appears in the corpus
    // This is needed for accurate unigram frequency calculations
    unigram_counts: HashMap<String, usize>,
    // Vocabulary: The set of unique words in the corpus
    // This is crucial for smoothing methods like Laplace and Lidstone
    // which need to know the vocabulary size to handle unseen words
    vocab: std::collections::HashSet<String>,
    // Test bigrams: A set of bigrams to test smoothing methods on
    // Includes seen, unseen but plausible, and completely unseen bigrams
    test_bigrams: Vec<(String, String)>,
    /// The total number of unigram tokens in the corpus (sum of all unigram counts)
    total_unigram_tokens: usize,
    /// A dictionary that contains the amount of repetitions for a certain amount of repetitions
    pub repetitions: HashMap<usize, usize>
}

impl Corpus {
    /// Initializes the Corpus struct, parses the sentences, and fills in all counts and vocab.
    pub fn new() -> Self {
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

        let total_unigram_tokens = unigram_counts.values().sum();

        // Repetitions, where HashMap<R1, Amount of reps for R1)
        // We want to save how many times a bigram of 1 repetition exists, how many times
        // a bigram of 2 repetitions exist, and so on
        let mut repetitions_vec: Vec<usize> = Vec::new();
        bigram_counts.iter().for_each(|bigram| {
            repetitions_vec.push(*bigram.1);
        });
        let mut repetitions: HashMap<usize, usize> = HashMap::new();
        repetitions_vec.iter().for_each(|rep| {
            let amount: usize = repetitions.get(rep).unwrap_or(&0).clone();
            if amount != 0 {
                repetitions.insert(*rep, amount + 1);
            } else {
                repetitions.insert(*rep, 1);
            }
        });


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
            bigram_counts,
            unigram_counts,
            vocab,
            test_bigrams,
            total_unigram_tokens,
            repetitions
        }
    }



    pub fn get_unigram_start_count(&self, word: &str) -> usize {
        // This should return the total unigram count of the word, not bigram counts
        // where the word appears as the first element
        *self.unigram_counts.get(word).unwrap_or(&0)
    }

    /// Returns the count of a bigram (word pair) in the corpus
    pub fn get_bigram_count(&self, bigram: &(String, String)) -> usize {
        self.bigram_counts.get(bigram).cloned().unwrap_or(0)
    }

    /// Returns the size of the vocabulary (number of unique words)
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Returns a reference to the test bigrams
    pub fn test_bigrams(&self) -> &Vec<(String, String)> {
        &self.test_bigrams
    }

    pub fn get_num_unique_continuations(&self, context: &str) -> usize {
        self.bigram_counts
            .keys()
            .filter(|(h, _)| h == context)
            .count()
    }

    /// Returns the context count c(h) - how many times the context word appears as the first word in bigrams
    pub fn get_context_count(&self, context: &str) -> usize {
        self.bigram_counts
            .iter()
            .filter(|((h, _), _)| h == context)
            .map(|(_, &count)| count)
            .sum()
    }

    /// Returns the total number of unigram tokens in the corpus
    pub fn get_total_unigram_tokens(&self) -> usize {
        self.total_unigram_tokens
    }

    /// Returns the total number of bigrams that exist in the corpus
    pub fn get_total_existing_bigrams(&self) -> usize {
        self.bigram_counts.values().sum()
    }

    // Returns the number of legal unseen bigrams, that's to say, bigrams that are not in the corpus
    // but are in the vocabulary
    pub fn get_legal_unseen_bigrams(&self) -> usize {
        let observed_types = self.bigram_counts.len();
        let total_possible = self.vocab_size() * self.vocab_size();
        total_possible - observed_types
    }


}
