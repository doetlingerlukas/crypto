mod factors;
use factors::factors;

extern crate num_bigint;
use num_bigint::BigUint;
extern crate num_traits;
use num_traits::ToPrimitive;

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

  println!("1 = {} - {} × ({} - {} × {}) = d × {} - {} × {}", e, factor_2, x, factor_1, e, e, factor_2, x);

  let d = (e - factor_2 * (x - factor_1 * e) + factor_2 * x) / e;
  println!("⇒ d = {}", d);

  let ciphers: &[u64] = &[2966, 0, 17830, 7105, 15200, 15200, 0];

  let decrypted: String = ciphers.iter()
    .map(|&c|  BigUint::from(c).modpow(&BigUint::from(d), &BigUint::from(n)).to_u64().unwrap())
    .map(|q| ('A' as u8 + q as u8) as char)
    .collect();

  println!();
  println!("Decrypted Cipher:");
  println!("{}", decrypted);
}
