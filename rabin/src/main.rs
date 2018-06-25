extern crate num_bigint;
use num_bigint::BigUint;
extern crate num_traits;
use num_traits::ToPrimitive;

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
  let (mut a1, mut a2, mut a3) = (a.abs(), 1, 0);
  let (mut b1, mut b2, mut b3) = (b.abs(), 0, 1);

  while a1 % b1 != 0 {
    let q = a1 / b1;

    let (c1, c2, c3) = (b1, b2, b3);

    b1 = a1 - q * c1;
    b2 = a2 - q * c2;
    b3 = a3 - q * c3;
    a1 = c1;
    a2 = c2;
    a3 = c3;
  }

  let u = a.signum() * b2;
  let v = b.signum() * b3;

  let g = u * a + v * b;

  (g, u, v)
}

fn positive_modulo(x: i64, n: i64) -> i64 {
  let mut i = x;

  while i < 0 {
    i += n;
  }

  i % n
}

fn encrypt(m: i64, k: (i64, i64)) -> i64 {
  let (n, b) = k;

  (m * (m + b)) % n
}

fn decrypt(c: i64, k: (i64, i64), l: (i64, i64)) -> i64 {
  let (n, b) = k;
  let (p, q) = l;

  let (_, u, v) = gcd(p, q);

  let m_p = BigUint::from(c as u64).modpow(&BigUint::from((p as u64 + 1) / 4), &BigUint::from(p as u64)).to_i64().unwrap();
  let m_q = BigUint::from(c as u64).modpow(&BigUint::from((q as u64 + 1) / 4), &BigUint::from(q as u64)).to_i64().unwrap();

  let r = positive_modulo(u * p * m_q + v * q * m_p, n);
  let r_ = n - r;
  let s = positive_modulo(u * p * m_q - v * q * m_p, n);
  let s_ = n - s;

  println!("c = {}", c);
  println!("c_r = {}",   encrypt(r, k));
  println!("c_r_ = {}",  encrypt(r_, k));
  println!("c_s = {}",   encrypt(s, k));
  println!("c_s_ = {}",  encrypt(s_, k));

  println!("r = {}",   r);
  println!("r_ = {}",  r_);
  println!("s = {}",   s);
  println!("s_ = {}",  s_);


  if encrypt(r, k) == c { r }
  else if encrypt(r_, k) == c { r_ }
  else if encrypt(s, k) == c { s }
  else { s_ }
}

fn main() {
  let p = 211;
  let q = 199;
  let b = 1234;
  let n = p * q;

  let k = (n, b);
  let l = (p, q);

  println!("{}", decrypt(27892, k, l));
  println!("{}", decrypt(27889, k, l));

  println!("{}", encrypt(16278, k));
  println!("{}", encrypt(13938, k));
}
