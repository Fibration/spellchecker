extern crate regex;

use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::io::{ self , Read };
use std::collections::HashMap;

fn main() {
    let freq_map = get_dictionary();
    println!("Got a dictionary.");

    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");


        if &guess == "QUIT" {
            break
        } else {
            println!("{}",get_correction(String::from(&guess[..]), &freq_map));
        }
    
        
    }
    
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

fn create_frequency_map(corpus: Vec<&str>) -> HashMap<String, u64> {
    corpus.into_iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(String::from(word)).or_insert(0) += 1;
        map
    })
}

fn get_dictionary() -> HashMap<String, u64> {
    let text: String = get_text("big.txt").unwrap().to_lowercase();
    let re = Regex::new(r"[[:^alpha:]]").unwrap();
    let stripped_text = re.replace_all(&text, " ");
    let words = get_words(&stripped_text);
    let all_words = words.into_iter().flatten().collect();

    create_frequency_map(all_words)
}

fn generate_corrections(word: &str) -> Vec<String> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    // let mut corrections: Vec<String> = Vec::new();
    // corrections.push(String::from(word));
    //let length = word.len();

    let mut corrections: Vec<String> = word.char_indices().map(|c| {
        let (left, right) = word.split_at(c.0);
        // deletes
        let del: String = format!("{}{}", &left, &right[1..]);
        let trans: String = if right.len() > 1 {
            format!(
                "{}{}{}{}", 
                &left,
                &right[1..2],
                &right[0..1],
                &right[2..]
            )
        } else {
            format!("{}{}", &left, &right)
        };
        let sub: Vec<String> = alphabet.iter().map(|&d| {
            format!("{}{}{}", &left, d, &right[1..])
        }).collect();
        let ins: Vec<String> = alphabet.iter().map(|&d| {
            format!("{}{}{}", &left, d, &right)
        }).collect();

        let mut corrections = vec![del, trans];
        corrections.extend(sub);
        corrections.extend(ins);
        corrections
    }).flatten().collect();

    //add the insertions at the end
    let end_insertions: Vec<String> = alphabet.iter().map(|&c| format!("{}{}", word, c)).collect();
    corrections.extend(end_insertions);
    corrections.sort();
    corrections.dedup();
    corrections
}

fn get_known(words: Vec<String>, freq_map: &HashMap<String,u64>) -> Vec<String> {
    println!("{:?}", freq_map["spelling"]);
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
    println!("{}", &word);
    let candidates = get_candidates(word, freq_map);
    println!("Candidates are {:?}", candidates);
    let index: usize = candidates.clone().into_iter().map(
        |candidate| freq_map[&candidate]
    ).enumerate().max_by_key(|&(_, item)| item).unwrap().0;
    println!("Candidate number {}.", index);
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
fn test_get_dictionary() {
    let dict: HashMap<String, u64> = get_dictionary();
    
    assert_eq!(
        4,
        dict["spelling"]
    )
}

#[test]
fn test_corrections() {
    let mut corr = vec!["he", "te", "th", "ahe", "tae", "tha", "bhe", "tbe", "thb", "che", "tce", "thc"];
    corr.sort();
    println!("{:?}",generate_corrections("speling"));
    assert_eq!(
        corr[..3].to_vec(),
        generate_corrections("the")[..3].to_vec()
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

#[test]
fn test_1() {
    let dict: HashMap<String,u64> = get_dictionary();
    
    assert_eq!(String::from("spelling"), get_correction(String::from("speling"), &dict));
}