fn main() {
  for i in 0..26 {
    println!("{} {}", "UJGUGNNUUGCUJGNNUDAVJGUGCUJQTG".chars()
      .map(|c| ((c as u8 - 'A' as u8 + 26 - i) % 26 + 'A' as u8) as char)
      .collect::<String>(), i);
  }
}
