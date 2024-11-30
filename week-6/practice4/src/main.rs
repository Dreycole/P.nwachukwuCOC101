use std::io;


fn add(a :i32, b:i32){

    let sum =a + b;
    println!("sum of a and b ={}",sum);

}

fn main(){
    let mut input1 = string::new();
    println!("enter input for parameter A:");
    io::stdin().read_line(&mut input 1).expect("failed to read input");
    let a:i32 = input1.trim().parse().expect("invalid input");

    let mut input2 =string::new();
    println!("enter input for parameter b:");
    io::stdin().read_input(&mut input2).expect("failed to read input");
    let b:i32 =input2.trim().parse().expect("invalid input");

    //call add function with arguments
    add(a,b);
}
