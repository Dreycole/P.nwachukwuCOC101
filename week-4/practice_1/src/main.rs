fn main() {
   println!{"\nstudent information management system!"};


   //input name
   println!{"\nplease enter your name"};
   let mut name= string::new();
   io::stdin()
   .read_line(& mut name)
   .expect("failed to read input");
print!(" your name is:{}", name);

//input age
println!("\nenter your age");
let mut age=string::new();
io::stdin().read_line(&mut age).expect("failed to read input");
let age:i32=age=trim().porse().expect("input not an interger");
println!("your age is :{}", age);
}
