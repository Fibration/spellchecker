extern crate regex;

use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::io::{ self , Read };

fn main() {
    let text: String = get_text("big.txt").unwrap().to_lowercase();
    let re = Regex::new(r"[[:^alpha:]]").unwrap();
    let stripped_text = re.replace_all(&text, " ");
    let words = get_words(&stripped_text);
    let all_words = words.into_iter().fold(vec![], |mut collection, line| {
        collection.extend(line);
        collection
    });
    println!("{:?}", all_words);
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