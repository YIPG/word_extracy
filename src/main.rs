use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("word.txt").expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something wrong with file");

    let v: Vec<&str> = content.split("\n").collect();
    let mut output: Vec<Vec<&str>> = Vec::new();
    for sentence in &v {
        let word_pair: Vec<&str> = sentence.split("、").collect();
        output.push(word_pair);
    }
    let output_json = File::create("word.json").expect("failed at creating file");

    serde_json::to_writer(output_json, &output).expect("failed at writing to json");
}
