fn main() {
    
    
    let fullname = " chidozie pascal nwachukwu";
    let department = "computer science";
    let uni = "pan atlantic university";

    let mut school = "school of science ".to string();
    //push string
    school.push_str("and technology");

    println!("my name is :{}", fullname);
    //check length
    println!("the length of my fullname is :{}", fullname);
    println!("i am a student of {} department", department);
    println!("{}",school);
    println!("{}", uni);
}
