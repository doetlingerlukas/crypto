struct PrimeIterator {
  n: u64,
  div: u64,
}

impl PrimeIterator {
  fn new(n: u64) -> Self {
    PrimeIterator { n: n, div: 2 }
  }
}

impl Iterator for PrimeIterator {
  type Item = Option<u64>;

  fn next(&mut self) -> Option<Option<u64>> {
    let div = self.div;

    if self.n < div {
      return None
    }

    self.div += 1;

    if self.n % div == 0 {
      self.n /= div;
      return Some(Some(div))
    }

    Some(None)
  }
}

pub fn factors(n: u64) -> Vec<u64> {
  PrimeIterator::new(n).into_iter().filter_map(|i| i).collect()
}
