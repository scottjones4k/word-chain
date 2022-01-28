use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Cli {
  pub start: String,
  pub end: String,
}

pub fn iterate (value: &str) -> String {
  let mut rng = rand::thread_rng();
  let number = rng.gen_range(0..value.chars().count());
  let replaced = replace_character(value, number, get_random_char());
  replaced
}

pub fn is_valid(value: &String, dict: &Vec<String>) -> bool {
  dict.contains(value)
}

pub fn score(value: &str, target: &str) -> usize{
  let source_chars = value.chars();
  let destination_chars = target.chars();
  source_chars.zip(destination_chars).filter(|(a,b)| a == b).count()
}

pub fn load_dictionary() -> Vec<String> {
    let mut dict = Vec::new();
    if let Ok(lines) = read_lines("./dictionary.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                dict.push(word);
            }
        }
    } else {
       println!("No file");
    }
    dict
}

pub fn get_chance() -> usize {
  let mut rng = thread_rng();
  rng.gen_range(0..100)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn replace_character(value: &str, index: usize, character: char) -> String {
  let mut first = value.get(0..index).unwrap().to_owned();
  let second = value.get(index+1..).unwrap().to_owned();
  first.push(character);
  first.push_str(&second);
  first
}

fn get_random_char() -> char {
  const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";
  let mut rng = thread_rng();
  let rand = rng.gen_range(0..CHARSET.chars().count());
  let character = CHARSET.chars().nth(rand).unwrap().to_owned();
  character
}
