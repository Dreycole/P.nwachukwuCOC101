fn main() {
   let n1 ="electrical".to_string();
   let n2 ="electronic".to_string();
   let n1 ="engineering".to_string();
   let n4 =n1 + &n2 +&n3;//n2 & n3 reference is passed

   //about electrical/electronic
   println!("\nthe {} is informed by the aspiration to train electrical
   /electronic engineering professionals in the areas of design, buildig and
   maintenance of electrical control systems,",);

   let w1 = "computer".to_string();
   let w2 = "science".to_string();
   let w3 = w1 + &w2; // the reference is passed
   println!();
   println!("{} is aimed at developing competent, creative,innovative,
   entrepreneurial and 
   ethically-minded person capable of creating value in the diverse fields of computer science.",w3);


}
