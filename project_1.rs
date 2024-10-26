fn main() {
   let p:f64=520_000_000.0;
   let r:f64= 10.0;
   let t:f64=2.0;

   // compound interest
   let a:f64 = p * (1.0 + (r/100.0)).powf(t);
   let ci = a - p;
   println!("{}",ci)
}