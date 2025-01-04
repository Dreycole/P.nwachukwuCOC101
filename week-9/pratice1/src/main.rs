 use std::io::write;


 fn main() {

    let announce="week 9 - rust file input & output\n";
    let dept=" department of computer science";


    let mut file= std::fs::file::create ("data.txt").expect("create failed");
    file.write_all("welcome to rust programming\n" .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");

    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\ndata written to file.");

    
 }
