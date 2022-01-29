extern crate word_chain;
use std::time::Instant;

fn main() {
  let start = std::env::args().nth(1).expect("no starting word given");
  let end = std::env::args().nth(2).expect("no ending word given");
  let args = word_chain::Cli {
    start: start,
    end: end
  };
  word_chain::validate_input(&args.start, &args.end);
  let dict = word_chain::load_dictionary(args.start.chars().count());
  let chance_same_score = 20;
  let chance_less_score = 10;
  let now = Instant::now();
  let mut next_word: String = args.start;
  let mut score = word_chain::score(&next_word, &args.end);
  let mut new_word: String;
  let mut new_score: usize;
  let mut results = Vec::new();
  results.push(next_word.clone());
  while score < args.end.chars().count() {
    new_word = word_chain::iterate(&next_word);
    new_score = word_chain::score(&new_word, &args.end);
    if word_chain::is_valid(&new_word, &dict) {
      if new_score > score || (new_score == score && word_chain::get_chance() > chance_same_score) || (new_score < score && word_chain::get_chance() > chance_less_score) {
        next_word = new_word;
        score = new_score;
        if results.contains(&next_word) {
          let index = results.iter().position(|r| r == &next_word).unwrap();
          results.truncate(index+1);
        } else {
          results.push(next_word.clone());
        }
      }
    }
  }
  results = word_chain::shorten_chain(results, 0);
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
  println!("{:?}", results);
}
