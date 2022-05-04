extern crate rand;

use std::path::Path;
use std::fs::File;
use std::vec::Vec;
use serde::Deserialize;
use rand::Rng;
mod classifier;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Intent {
    slug: String,
    cases: Vec<String>,
    answers: Vec<String>
}
fn main() {
    //JSON reading
    let json_file_path = Path::new("src/dataset.json");
    let file = File::open(json_file_path).expect("file not found");
    let mut intents:Vec<Intent> = serde_json::from_reader(file).expect("error while reading");
    let mut nbc = classifier::NaiveBayesClassifier::new();
    // Create iterator
    let  it = &mut intents;
    for intent in &mut *it {
        for case in &mut *intent.cases {
            //Normalize Text
            let case = case.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '¿'][..], "")
                        .to_lowercase();
            //Train model
            nbc.train(&case, &intent.slug);
        }
    }
    let text = "motomami"
            .replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '¿'][..], "")
            .to_lowercase();
    let result = nbc.guess(&text);

    println!("Result: {}", result);

    for intent in &mut *intents {
        if result == intent.slug {
            let index: usize = rand::thread_rng().gen_range(0, intent.answers.len());
            let ref answer = &intent.answers[index];
            println!("Answer: {}", answer);
        }
    }
}