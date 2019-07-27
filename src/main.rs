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

fn generate_corrections(word: &str) -> Vec<String> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut corrections: Vec<String> = Vec::new();
    corrections.push(String::from(word));
    //let length = word.len();

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
        }).collect::<Vec<String>>()
    }).flatten().collect();
    corrections.extend(subs);
    
    corrections.sort();
    corrections.dedup();
    corrections
}

fn get_known(words: Vec<String>, freq_map: &HashMap<String,u64>) -> Vec<String> {
    words.into_iter().filter(|word| freq_map.contains_key(&word[..])).map(|word| word).collect()
}

fn get_candidates(word: String, freq_map: &HashMap<String,u64>) -> Vec<String> {
    if !get_known(vec![word.clone()], &freq_map.clone()).is_empty() {
        vec![word]
    } else {
        get_known(generate_corrections(&word), &freq_map.clone())
    }
}

fn get_correction(word: String, freq_map: &HashMap<String,u64>) -> String {
    let candidates = get_candidates(word, freq_map);
    let index: usize = candidates.clone().into_iter().map(
        |candidate| freq_map[&candidate]
    ).enumerate().max_by_key(|&(_, item)| item).unwrap().0;
    candidates[index].clone()
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
        generate_corrections("the")[..5].to_vec()
    );
}

#[test]
fn test_get_correction() {
    let mut freq_map: HashMap<String, u64> = HashMap::new();
    freq_map.insert(String::from("free"), 10);
    freq_map.insert(String::from("freed"), 4);

    assert_eq!(
        String::from("free"),
        get_correction(String::from("freee"), &freq_map)
    );
}