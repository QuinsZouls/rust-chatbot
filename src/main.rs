use std::path::Path;
use std::fs::File;
use std::vec::Vec;
use serde::Deserialize;

mod classifier;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Intent {
    slug: String,
    cases: Vec<String>,
    answers: Vec<String>
}
fn main() {
    println!("Hello, world!");
    //Leemos el json
    let json_file_path = Path::new("src/dataset.json");
    let file = File::open(json_file_path).expect("file not found");
    let intents:Vec<Intent> = serde_json::from_reader(file).expect("error while reading");
    let mut nbc = classifier::NaiveBayesClassifier::new();
    for intent in intents {
        for case in intent.cases {
            //Normalizamos el texto
            let case = case.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '¿'][..], "")
                        .to_lowercase();
            //Entrenamos el modelo
            nbc.train(&case, &intent.slug);
        }
    }
    let result = nbc.guess("necesito su ayuda ayudar?");

    println!("Result: {}", result)
}
