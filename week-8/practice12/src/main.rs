fn main() {
    
    // mutable array
    let mut colors = ["red","greem","yellow","white"];fn

    println!("\nOriginal array = {:?}", colors);


    //mutable slice
    let sliced_colors =&mut colors[1..3];

    println!("first slice ={:?}",sliced_colors);

    //change the value of the original slice at the first index 
    sliced_colors[1] = "purple";

    println!("changed slice ={:?}",sliced_colors);
}
