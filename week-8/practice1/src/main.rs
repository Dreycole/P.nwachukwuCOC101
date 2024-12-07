fn main() {

    //using vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\nThe length of the Vec::new is:{}",v.len());

    //using macro
    let v =vec!["grace", "effiong", "basil", "kareem", "susan"];

    //printing the size of the vector
    println!("\nThe length of the vec macro is:{ }",v.len());
}
