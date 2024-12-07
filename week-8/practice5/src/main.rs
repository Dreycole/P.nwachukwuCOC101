fn main() {
    //create an empty vector "city"
    let mut city:vec<string> =Vec::new();


    //print city vector 
    println!("the city vector has element {}",city.len());


    //push new elements into
    let mut input1= string::new();
    println!("how many cities do you want to enter ");
    std::io::stdin().read_line(&mut input1).expect("failed to read input ");
    let city_num:i32 =input1.trim().parse().expect("invalid input");
    for count in 0..city_num{
        let mut input2 = string::new();
        println!("enter city {}", count+1);
        let new_city:string = input2.trim().parse().expect("failed to read input");
        city.push(new_city);


    }

    println!("your preferred cities are :\n");
    let mut count+1;

    //loop to iterate elements in vector 
    for i in city 

    {

        //iterating through i on the vector
        println!("{} {}", count ,i);
        count+=1;
    }
}
