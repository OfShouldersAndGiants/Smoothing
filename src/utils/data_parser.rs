use std::collections::HashMap;
use csv::Reader;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use bincode;
use crate::utils::tokenizer::create_string_tokens;

#[derive(Serialize, Deserialize)]
pub struct DataParser {
    pub total_words: usize,
    pub unique_words: usize,
    pub ham_tokens: HashMap<u32, usize>,
    pub spam_tokens: HashMap<u32, usize>,
}

impl DataParser {
    pub fn new(file_path: String, cache_path: &str) -> Self {
        // Check if cached data exists
        if let Ok(cached) = Self::load_from_cache(cache_path) {
            println!("Loaded data from cache: {}", cache_path);
            return cached;
        }
        let mut ham_tokens = HashMap::new();
        let mut spam_tokens = HashMap::new();
        let mut total_words = 0;

        // First, count the number of records for the progress bar
        let mut rdr = Reader::from_path(&file_path).expect("Failed to open CSV file");
        let total_records = rdr.records().count();
        // Rewind the reader
        let mut rdr = Reader::from_path(&file_path).expect("Failed to open CSV file");
        // Create progress bar
        let pb = ProgressBar::new(total_records as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("#>-"));

        // Iterate over the lines of the csv file
        for result in rdr.records() {
            // Check if the line is valid else skip
            let record = match result {
                Ok(rec) => rec,
                Err(_) => { pb.inc(1); continue; },
            };
            // Skip header if present too
            if record.get(0) == Some("label") { pb.inc(1); continue; }

            // Parse the label and email
            let label = match record.get(0).and_then(|s| s.parse::<i32>().ok()) {
                Some(l) => l,
                None => { pb.inc(1); continue; },
            };
            let email = match record.get(1) {
                Some(e) => e,
                None => { pb.inc(1); continue; },
            };

            // Encode the email words with the bpe tokenizer
            let word_tokens = create_string_tokens(email);
            total_words += word_tokens.len();

            // If the label is 1(a normal important email), add the word tokens to the ham words
            if label == 1 {
                for token in word_tokens {
                    *ham_tokens.entry(token).or_insert(0) += 1;
                }
            } else {
                // If the label is 0(a spam email), add the word tokens to the spam words
                for token in word_tokens {
                    *spam_tokens.entry(token).or_insert(0) += 1;
                }
            }

            // Increment the progress bar
            pb.inc(1);
        }

        // Finish the progress bar
        pb.finish_with_message("Done parsing CSV");

        // Calculate the unique words
        let unique_words: usize = ham_tokens.keys().chain(spam_tokens.keys()).collect::<std::collections::HashSet<_>>().len();

        // Create the parser
        let parser = Self {
            total_words,
            unique_words,
            ham_tokens,
            spam_tokens,
        };

        // Save to cache to prevent re-parsing the data. In case of want to re-parse the data, we can delete the cache file.
        parser.save_to_cache(cache_path).expect("Failed to save cache");
        println!("Saved data to cache: {}", cache_path);
        parser
    }

    fn save_to_cache(&self, cache_path: &str) -> std::io::Result<()> {
        let mut file = File::create(cache_path)?;
        let encoded = bincode::serialize(self).expect("Serialization failed");
        file.write_all(&encoded)?;
        Ok(())
    }

    fn load_from_cache(cache_path: &str) -> std::io::Result<Self> {
        let mut file = File::open(cache_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decoded: Self = bincode::deserialize(&buffer).expect("Deserialization failed");
        Ok(decoded)
    }
}
