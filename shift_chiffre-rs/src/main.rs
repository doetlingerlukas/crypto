fn main() {
  let chiffre_text = "UJGUGNNUUGCUJGNNUDAVJGUGCUJQTG";
  let a = 'A' as u8;

  for i in 0..26 {
    println!("{}", chiffre_text.chars().map(|c| ((c as u8 - a + i) % 26 + a) as char).collect::<String>());
  }
}
