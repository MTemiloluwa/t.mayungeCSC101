fn main() {
    let mut num:i8 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is: {}", num);

}

fn mutate_num_to_zero(param_num:&mut i8){
    *param_num = *param_num*0; //the refernce
    println!("param_num value is: {}", param_num);
}