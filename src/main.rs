extern crate regex;

use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::io::{ self , Read };
use std::collections::HashMap;

fn main() {
    // read the Gutenberg sample, clean it of punctuation and get the words from it
    let text: String = get_text("big.txt").unwrap().to_lowercase();
    let re = Regex::new(r"[[:^alpha:]]").unwrap();
    let stripped_text = re.replace_all(&text, " ");
    let words = get_words(&stripped_text);

    // the lines are embedded as vectors within a larger vector
    // need to flatten the vector
    let all_words = words.into_iter().flatten().collect();

    // create a hashmap counting occurences of each word
    let freq_map = create_frequency_map(all_words);

}

fn get_text(s: &str) -> io::Result<String> {
    let path = Path::new(s);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn get_words<'a>(text: &'a str) -> Vec<Vec<&'a str>> {
    text.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

fn create_frequency_map<'a>(corpus: Vec<&'a str>) -> HashMap<&'a str, u64> {
    corpus.iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(word).or_insert(0) += 1;
        map
    })
}

fn generate_possible_corrections(word: &str) -> Vec<String> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut corrections: Vec<String> = vec![];
    let length = word.len();

    // deletes
    let dels: Vec<String> = word.char_indices().map(|letter| {
        word.char_indices().filter(|&c| c!=letter).map(|c| c.1).collect()
    }).collect();
    corrections.extend(dels);

    // substitutions
    let subs: Vec<String> = alphabet.iter().map(|&c| {
        word.char_indices().map(|d| {
            word.char_indices().map(|e| {
                if e == d {
                    c
                } else {
                    e.1
                }
            }).collect()
        }).collect()
    }).into_iter().flatten().collect();
    corrections.extend(subs);

    corrections
}

fn get_closest(word: &str, freq_map: &HashMap<String,u64>) -> String {
    let mut closest: String = String::from(word);
    
    closest
}

#[test]
fn test_get_text() {
    let s = "big.txt";
    assert_eq!(
        "The Project Gutenberg EBook of The Adventures of Sherlock Holmes",
        get_text(s).unwrap().lines().next().unwrap()
    );
}

#[test]
fn test_get_words() {
    let text = vec!["the","project","gutenberg","ebook"];
    let mut big_text = String::from("The Project Gutenberg EBook of The Adventures");
    big_text = big_text.to_lowercase();
    let words = get_words(&big_text);

    assert_eq!(
        text[..],
        words[0][..4]
    );
}

#[test]
fn test_vec_to_hash() {
    let vec = vec!["there", "were", "many", "many", "issues"];
    let hash = create_frequency_map(vec);

    assert_eq!(2 as u64, hash["many"]);
}

#[test]
fn test_corrections() {
    let corr = vec!["he", "te", "th", "ahe", "tae"];
    assert_eq!(
        corr,
        generate_possible_corrections("the")[..5].to_vec()
    );
}
