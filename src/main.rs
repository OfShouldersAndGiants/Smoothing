use data::corpus::Corpus;
use smoothing::{absolute_discount, good_turing, kneser_ney, laplace, lidstone};

mod data;
mod smoothing;

fn main() {
    // 1. The first thing we do is define our corpus using the Corpus struct
    //    All parsing, counting, and test bigram logic is encapsulated in Corpus
    let corpus = Corpus::new();

    // 2. Get the vocabulary size for use in smoothing methods
    let vocab_size = corpus.vocab_size();

    // 2.1 Get the frequencies for each bigram count
    let frequencies = corpus.repetitions.clone();

    // 2.2 Get the total number of distinct bigram types
    let total_distinct_bigram_types = corpus.get_total_distinct_bigram_types();

    // 3. For each test bigram, apply smoothing methods
    //    We print the results for Laplace and Lidstone smoothing
    println!("Bigram\t\t     Laplace  Lidstone AbsDiscount GoodTuring Kneser-Ney");

    for bigram in corpus.test_bigrams() {
        // Set up our variables for the smoothing methods
        let context = &bigram.0; // The first word in the bigram
        let word = &bigram.1; // The second word in the bigram
        let context_count = corpus.get_context_count(context); // How many times the context appears as first word in bigrams
        let bigrams_count = corpus.get_total_existing_bigrams(); // How many existing bigrams exist in the corpus
        let bigram_count = corpus.get_bigram_count(bigram); // How many times the bigram appears
        let num_unique_continuations = corpus.get_num_unique_continuations(context); // How many unique words follow the context
        let unigram_count_w2 = corpus.get_unique_predecessors_of_w2(word); // How many times the word appears

        let unigram_count_w = corpus.get_unigram_start_count(word); // How many times the word appears
        let total_unigram_tokens = corpus.get_total_unigram_tokens(); // How many total tokens in the corpus

        let discount = 0.75; // The discount parameter for absolute discount smoothing
        let legal_unseen_bigrams = corpus.get_legal_unseen_bigrams();

        // Laplace smoothing is a simple method that adds 1 to all counts and divides by (context_count + vocab_size)
        let laplace = laplace::laplace_smooth(bigram_count, context_count, vocab_size);
        // Lidstone smoothing is a more flexible method that uses a parameter λ (here we use 0.5)
        let lidstone = lidstone::lidstone_smooth(bigram_count, context_count, vocab_size, 0.5);
        // Absolute discount smoothing is a more flexible method that uses a parameter D (here we use 0.75)
        let absolute_discount = absolute_discount::absolute_discount_smooth(
            bigram_count,
            context_count,
            num_unique_continuations,
            unigram_count_w,
            total_unigram_tokens,
            discount,
        );
        // Good turing smoothing...
        let good_turing = good_turing::good_turing(
            bigrams_count as f64,
            bigram_count,
            &frequencies,
            legal_unseen_bigrams as f64,
        );
        // Kneser-Ney smoothing...
        let kneser_ney = kneser_ney::kneser_ney(
            bigram_count,
            context_count,
            num_unique_continuations,
            unigram_count_w2,
            total_distinct_bigram_types,
            discount,
        );

        // A simple visualization of the results ;)
        println!(
            "{:<20} {:<8.4} {:<8.4} {:<8.4} {:<8.4} {:<8.4}",
            format!("({:?}, {:?})", context, word),
            laplace,
            lidstone,
            absolute_discount,
            good_turing,
            kneser_ney,
        );
    }
}
