# Smoothing: Rust Torch Project

This is a Rust project that uses PyTorch (through `tch-rs`) for AI and machine learning development.

## What is Laplace Smoothing?

Laplace smoothing (also known as additive smoothing) is a technique used in probability and statistics to handle the problem of zero probabilities in categorical data. It is commonly used in natural language processing, such as in Naive Bayes classifiers, to ensure that every possible event has a nonzero probability.

The formula for Laplace smoothing is:

    P(word|class) = (count(word, class) + 1) / (count(class) + V)

Where:
- `count(word, class)` is the number of times the word appears in documents of the given class
- `count(class)` is the total number of words in documents of the given class
- `V` is the number of unique words in the vocabulary

## Why is it important? The Zero-frequency problem
When a word is not present in the training data (for example, in spam emails), its probability would be zero. Due to the multiplicative nature of probability calculations, this zero probability would force the entire email's probability of being spam to zero, regardless of other strong spam indicators. This leads to incorrect classifications and reduces the model's effectiveness.

## The solution
Laplace smoothing solves the zero-frequency problem by adding a small constant (typically 1) to all word counts, including words that haven't appeared in the training data. This ensures that no probability is exactly zero, while still maintaining the relative frequencies of observed words. The added constant acts as a prior probability, suggesting that even if we haven't seen a word in our limited training data, there's still a small chance it could appear. This prevents the model from being overly confident about the absence of words and allows it to make more robust predictions when encountering new, previously unseen words.

### Example
Suppose you have the following word counts for a class:
- "apple": 2
- "banana": 1

And your vocabulary consists of three words:
- "apple"
- "banana"
- "orange"

**Input:**
- Counts: {"apple": 2, "banana": 1}
- Vocabulary: ["apple", "banana", "orange"]

**Apply Laplace smoothing:**
- P(apple) = (2 + 1) / (3 + 3) = 0.5
- P(banana) = (1 + 1) / (3 + 3) ≈ 0.333
- P(orange) = (0 + 1) / (3 + 3) ≈ 0.167

**Expected Output (after Laplace smoothing):**
- P(apple) = 0.5
- P(banana) = 0.333
- P(orange) = 0.167

This shows that even though "orange" did not appear in the counts, it still receives a nonzero probability due to Laplace smoothing.

## Limitations of Laplace Smoothing

While Laplace smoothing is a valuable technique for handling zero probabilities, it comes with several important limitations:

1. **Uniform Prior Assumption**:
   - The technique assumes all features (words) have equal prior probabilities
   - In reality, some words are naturally more common than others
   - This uniform assumption can lead to suboptimal probability estimates

2. **Fixed Alpha Value**:
   - Uses a constant smoothing parameter (typically α = 1) for all features
   - Different features might benefit from different smoothing intensities
   - One-size-fits-all approach may not be optimal for all scenarios

3. **Vocabulary Size Sensitivity**:
   - The denominator includes the vocabulary size (V)
   - For large vocabularies, this can lead to over-smoothing
   - Probabilities might become too uniform when V is very large

These limitations have led to the development of alternative smoothing techniques like:
- Good-Turing smoothing
- Kneser-Ney smoothing
- Witten-Bell smoothing

Choose the smoothing technique based on your specific use case and requirements.


## Project Setup

1. Make sure you have Rust installed. If not, install it using:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd laplace
   ```

3. The project uses specific versions of dependencies, particularly:
   - `tch = "0.19.0"` (PyTorch bindings for Rust)
   Make sure not to change this version as it's compatible with the current libtorch setup.


### Installing libtorch on macOS

The good thing is that there's no need to install libtorch manually and globally. Simply run:
```bash
make setup
```

This will automatically:
1. Download the appropriate version of libtorch for macOS
2. Extract it to the correct location within this project
3. Set up the necessary environment variables in the .cargo/config.toml file

That way this project is self-contained, doesn't require any global installations and you don't have to worry about version conflicts.

## Building the Project

After setting up libtorch and the environment variables, you can build the project:

```bash
cargo build
```

## Project Structure

This project is set up for AI and machine learning development using PyTorch in Rust. It provides a foundation for building:
- Machine Learning models
- Neural Networks
- Deep Learning applications
- AI inference and training pipelines

## Troubleshooting

If you encounter any linking errors open an issue on GitHub and I'll help you out.

## License

MIT License
