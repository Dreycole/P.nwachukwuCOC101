fn main() {
  let mut num:i32 =5;
  mutate_num_to _zero(&mut num) ;
  println!("the value of num is:{}",num);

}

fn mutate_num_to_zero(param_num:&mut i32){
    *param_num = *param_num*0;//de refernce
    println!("param_num_value is:}{}",param_num);
}
