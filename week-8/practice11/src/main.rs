fn main() {


    //let arrays of numbers 
    let numbers =[1,2,3,4,5,];
    println!("original array = {:?}",number);

    //create a slice of 2nd and 3rd element 
    let slice1 = &numbers[1..3];
    prntln!("2nd and 3rd elements sliced ={:?}",slice1);


    //omit the start index
    let slice2 =&numbers[..3];

    //this means the slice starts from index 0 and goes up to index 3(exclusive)
    println!("index 0 to 3 sliced = {:?}",slice2);


//omit the start index
    let slice2 =&numbers[2..];

    //this means the slice starts from index 2 and goes up to index 5(exclusive)
    println!("index 2 to 5 sliced = {:?}",slice3);


//reference the whole array
//omit the start index
    let slice2 =&numbers[..];

    //this means the slice starts from index 0 and goes up to index 5(exclusive)
    println!("index 0 to 5 sliced = {:?}",slice4);

}
