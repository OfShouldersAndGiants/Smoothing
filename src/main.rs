use smoothing::{laplace, lidstone};
use data::corpus::Corpus;
mod smoothing;
mod data;

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
