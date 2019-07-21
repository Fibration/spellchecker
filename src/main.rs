use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{ self , Read };

fn main() {
    let text: String = get_text("big.txt").unwrap();

}

fn get_text(s: &str) -> io::Result<String> {
    let path = Path::new(s);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}


#[test]
fn test_get_text() {
    let s = "big.txt";
    assert_eq!(
        "The Project Gutenberg EBook of The Adventures of Sherlock Holmes",
        get_text(s).unwrap().lines().next().unwrap()
    );
}