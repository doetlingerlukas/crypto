use std::env;

extern crate rgsl;
use rgsl::{rng, Rng, RngType};

use rgsl::randist::binomial::{binomial};

fn main() {
  // To use another generator, run
  //   env GSL_RNG_TYPE=taus cargo run --release <sample_count>
  RngType::env_setup();

  let rng_type = rng::default();
  let mut rng = Rng::new(&rng_type).unwrap();

  let args: Vec<String> = env::args().collect();

  println!("Seed: {}", Rng::default_seed());
  println!("Generator: {}", rng.get_name());

  let sample_count: u64 = args.get(1).map_or(None, |x| x.parse().ok()).unwrap_or(1);

  let n = 20;
  println!("Wiederholungszahl: {}", n);
  let p = 0.2;
  println!("Erfolgswahrscheinlichkeit: {}", n);

  let samples: Vec<u32> = (0..sample_count).map(|_| {
    binomial(&mut rng, p, n)
  }).collect();
  println!("Stichproben: {:?}", samples);

  let x_ = samples.iter().sum::<u32>() as f64 / sample_count as f64;
  println!("Stichprobenmittel: {}", x_);

  let s2 = samples.iter().map(|xi| (*xi as f64 - x_).powf(2.0)).sum::<f64>() / (sample_count - 1) as f64;
  println!("Stichprobenstreuung: {}", s2);

  let e = n as f64 * p; // Beispiel 1.26
  println!("Erwartungswert: {}", e);

  let v = n as f64 * p * (1.0 - p); // Beispiel 1.26
  println!("Varianz: {}", v);
}
