use utils::data_parser::DataParser;
use utils::test_email_reader::read_test_email;
use std::env;
use smoothing::laplace::Laplace;
mod utils;
mod smoothing;

fn main() {
    // Get the path to the email file from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_email_txt>", args[0]);
        std::process::exit(1);
    }
    let email_file_path = &args[1];

    // This file is a csv with the following columns:
    // 0: (1 or 0) - ham or spam
    // 1: email text
    let file_path = "./src/assets/combined_email_data.csv";

    // The data parser will parse the data and return a struct with the ham and spam emails
    let data_parser = DataParser::new(file_path.to_string(), "cache.bin");

    // Get the test email tokens so we can use it to predict the truthfulness of the email
    let test_email_tokens = read_test_email(email_file_path);

    // Apply the Laplace smoothing to the test email tokens to get the probability of the email being ham or spam
    let laplace = Laplace::new(&data_parser, 1.0, &test_email_tokens);
    let prob_ham = laplace.log_prob_ham();
    let prob_spam = laplace.log_prob_spam();

    // If the probability of the email being ham is greater than the probability of the email being spam, then the email is ham
    if prob_ham > prob_spam {
        println!("The email is ham with a difference of {:.3}.", prob_ham - prob_spam);
    } else {
        println!("The email is spam with a difference of {:.3}.", prob_spam - prob_ham);
    }
}
