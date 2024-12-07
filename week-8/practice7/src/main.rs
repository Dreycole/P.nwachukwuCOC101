fn main() {
    // initialization of tuple with data type
    let datatype_tuple: (&str,f32,u8) =("rust", 3.14.100);
    println!("tuple contents= {:?}", tuple);

    // intialization of tuple without data type 
    let datatype_no tuple =("rust", "fun" 100);
    println!("tuple contents = {:?}",tuple);

    //accessing tuple element at index 0
    println!("value at index 0 = {}", datatype_tuple.0);

    //accessing tuple element at index 1
    println!("value at index 1 = {}", datatype_tuple.1);


    //accessing tuple element at index 2
    println!("value at index 2 = {}", datatype_tuple.2);
}
