fn main() {
    let num:i8 =5;
    mutate_num_to_zero(num);
    println!("The value of number is: {}", num);

}

fn mutate_num_to_zero(mut param_num: i8){
    param_num = param_num*0;
    println!("param_num value is: {}", param_num);
}