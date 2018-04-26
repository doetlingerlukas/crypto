extern crate ngrams;
use ngrams::Ngram;

extern crate rayon;
use rayon::prelude::*;

extern crate reqwest;
use reqwest::get;

extern crate regex;
use regex::Regex;

extern crate itertools;
use itertools::Itertools;

fn escape(string: &str) -> String {
  let lowercase_string = string.to_ascii_lowercase();
  let removed_non_ascii_lowercase = Regex::new(r"[^a-z ]").unwrap().replace_all(&lowercase_string, "").to_string();
  let merged_spaces = Regex::new(r"\s+").unwrap().replace_all(&removed_non_ascii_lowercase, " ").to_string();
  merged_spaces
}

fn ngram(string: &str, n: usize) -> Vec<Vec<char>> {
  let char_iter = string.par_chars();

  if n == 1 {
    return char_iter.map(|s| vec![s]).collect()
  }

  char_iter.collect::<Vec<_>>().into_iter().ngrams(n).collect()
}

fn monogram(string: &str) -> Vec<Vec<char>> {
  ngram(string, 1)
}

fn bigram(string: &str) -> Vec<Vec<char>> {
  ngram(string, 2)
}

fn trigram(string: &str) -> Vec<Vec<char>> {
  ngram(string, 3)
}

fn rank_ngrams(ngrams: &Vec<Vec<char>>) -> Vec<(String, usize)> {
  ngrams.iter().map(|v| (v, 1)).into_group_map().iter()
    .map(|(k, v)|(k.into_iter().collect(), v.len()))
    .sorted_by(|&(_, a), &(_, b)| b.cmp(&a))
}

fn entropy(vec: &Vec<(String, usize)>) -> f64 {
  let n = vec.iter().fold(0.0, |acc, &(_, count)| acc + count as f64);

  -vec.iter().map(|&(_, count)| {
    let p = count as f64 / n;
    if p == 0.0 { return 0.0; }
    p * p.log2()
  }).fold(0.0, |acc, e| acc + e)
}

fn main() {
  let text = get("http://www.gutenberg.org/files/135/135-0.txt").expect("Could not fetch URL.")
    .text().expect("No text found.");

  let escaped_text = escape(&text);

  let mono = rank_ngrams(&monogram(&escaped_text));
  let bi   = rank_ngrams(&bigram(&escaped_text));
  let tri  = rank_ngrams(&trigram(&escaped_text));

  let mono_entropy = entropy(&mono);
  let bi_entropy = entropy(&bi);
  let tri_entropy = entropy(&tri);

  println!("Monogram Entropy: {}", mono_entropy);
  for (ngram, count) in mono.into_iter().take(10) {
    println!("“{}”: {}", ngram, count);
  }

  println!();

  println!("Bigram Entropy: {} ({})", bi_entropy, bi_entropy / 2.0);
  for (ngram, count) in bi.into_iter().take(10) {
    println!("“{}”: {}", ngram, count);
  }

  println!();

  println!("Trigram Entropy: {} ({})", tri_entropy, tri_entropy / 3.0);
  for (ngram, count) in tri.into_iter().take(10) {
    println!("“{}”: {}", ngram, count);
  }
}
