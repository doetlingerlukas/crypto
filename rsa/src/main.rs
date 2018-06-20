mod factors;
use factors::factors;

fn main() {
  let n: u64 = 20687;
  let e: u64 = 31;

  let factors = factors(n);

  let (p, q) = (factors[0], factors[1]);

  println!("Factors:");
  println!("{} = {} × {}", n, p, q);
  println!();

  let x = (p - 1) * (q - 1);

  println!("{} = {} × {}", x, p - 1, q - 1);
  println!();

  let factor_1 = (x / e) as u64;
  let rest_1 = x - factor_1 * e;

  let factor_2 = (e / rest_1) as u64;
  let rest_2 = e - factor_2 * rest_1;

  let factor_3 = rest_1 / rest_2;
  let rest_3 = rest_1 - factor_3 * rest_2;

  println!("Eucledian Algorithm:");
  println!("{} = {} × {} + {}", x, factor_1, e, rest_1);
  println!("{} = {} × {} + {}", e, factor_2, rest_1, rest_2);
  println!("{} = {} × {} + {}", rest_1, factor_3, rest_2, rest_3);
  println!();

  if rest_2 != 1 {
    panic!{"gcd({}, {}) is not 1!", e, x};
  }

  println!();
  println!("1 = {} - {} × ({} - {} × {}) = d × {} - {} × {}", e, factor_2, x, factor_1, e, e, factor_2, x);

  let d = (e - factor_2 * (x - factor_1 * e) + factor_2 * x) / e;
  println!("⇒ d = {}", d);

  let ciphers = [2966, 0, 17830, 7105, 15200, 15200, 0];

  let decrypted = ciphers.iter().map(|&c| (c as u64).pow(d as u32) % n).map(|q| ((q as u8 + 65) as char).to_string()).collect::<Vec<String>>().join("");
  println!("{:?}", decrypted);
}
