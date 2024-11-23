use std::io;

//define food items

fn main() {
     // Display the menu
     println!("Menu:");
     println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
     println!("F = Fried Rice & Chicken - N3,000");
     println!("A = Amala & Ewedu Soup - N2,500");
     println!("E = Eba & Egusi Soup - N2,000");
     println!("W = White Rice & Stew - N2,500");

     //prices
     let prices=[("p",3_200),("f",3_000),("p",2_500),("p",2_000),("p",2_500)];

    //input food types
    let mut food_type = string::new();
    println!("\nEnter the type of food (P/F/A/E/W ):");
    io::stdin().read_line(&mut food_type).expect("failed to read input");
    let food_type =food_type.trim().to_uppercase();

    //validate food type
    let price = prices.item().find(|&&(code, )|code==food_type);
    if price.is_none(){
        println!("invalid food type selected!");
        return;
    }

    let price = price.unwrap().1;


//input quantity
let mut quantity =string::new();
println!("enter the quantity").expect("failed to read input");
io::stdin().read_line(&mut quantity).expect("failed to read input");
let the quantity:usize =match quantity.trim().parse(){
ok(num)=>num,
err(_)=>{
    println("invalid quantity entered!");
    return;
}

};

//calculate total cost
let total=price=quantity;
let discount=if total>10_000{total/20}else{0};
let final_total=total-discount;


//display the total charge
println!("\n--order summary----");
println!("food type:{}",food_type);
println!("quantity:{},"quantity);
println!("total:N{}",total);
if discount>0{
    println!("discount(5%): -N{}",discount);

}
println!("final total:N{}",final_total);
}
