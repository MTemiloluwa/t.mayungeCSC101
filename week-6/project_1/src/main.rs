use std::io;


fn main() {
    
//State the prices of food items

    let p:f32 = 32_00.0;
    let f:f32 = 3_000.0;
    let a:f32 = 25_00.0;
    let e:f32 = 2_000.0;
    let w:f32 = 25_00.0;
    
  

// display the menu for the resturant
    println!("     Resturant Menu    "); // display the menu
    println!("Code  Menu Item                    Price(N)");
    println!("P     Poundo Yam / Edinkaiko Soup  N{}", p);
    println!("F     Fried Rice & Chicken         N{}", f);
    println!("A     Amala & Ewedu Soup           N{}", a);
    println!("E     Eba & Egusi Soup             N{}", e);
    println!("W     White Rice & stew            N{}", w);



//let customer input the food code
println!("Welcome dear customer, please enter a code (P, F, A, E, W):");
let mut code = String::new();
io::stdin().read_line(&mut code).expect("Failed to read input");
let code = code.trim().to_uppercase();


//find price
let price:f32 = if code == "P" {
   p
}

else if code == "F" {
    f
}
else if code =="A"{
    a
}
else if code == "E"{
    e
}
else if code == "W"{
    w
}
else {
    println!("Code provided is invalid! Please enter a valid code");
    return;
};
println!("price = N{}", price);


//get quantity of food wanted by user
println!("Enter quantity of food wanted:");
let mut quantity_7 = String::new();
io::stdin().read_line(&mut quantity_7).expect("Failed to read quantity");
let quantity:f32 = quantity_7.trim().parse().expect("Please enter a valid number");

//Calculae the total
let total = price * quantity;
let mut final_total = total;

//ensure that 5% discount is given to customers whose total exceed 10_000
if total > 10_000.0{
    let discount = total * 0.05;
    final_total = total - discount;
    println!("\nDiscount applied (5%): N{}", discount);

}
// the total output result
println!("Total amount to pay: N{}", final_total);






}
