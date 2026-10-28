struct Laptops{
    lpbrand:String,
    qtstock:u32,
    qtbuy:u32,
    unit_price:u32,
}
impl Laptops{
    fn unit_cost(&self)->u32{
        self.qtbuy * self.unit_price
    }
}

fn main() {
    let laptop_1 = Laptops{
       lpbrand:String::from("HP"),
       qtstock:10,
       qtbuy:3,
       unit_price:650_000,
    };
    println!("\nBrand name is {} \n Quantity in stock is {} \n Quantity to buy is {}
        Price per one unit is {} \n Line total is {} ", laptop_1.lpbrand,laptop_1.qtstock,
        laptop_1.qtbuy,laptop_1.unit_price,laptop_1.unit_cost());
    
    let laptop_2 = Laptops{
       lpbrand:String::from("IBM"),
       qtstock:6,
       qtbuy:3,
       unit_price:755_000, 
    };
        println!("\nBrand name is {} \n Quantity in stock is {} \n Quantity to buy is {}
        Price per one unit is {} \n Line total is {} ", laptop_2.lpbrand,laptop_2.qtstock,
        laptop_2.qtbuy,laptop_2.unit_price,laptop_2.unit_cost());

    let laptop_3 = Laptops{
       lpbrand:String::from("Toshiba"),
       qtstock:10,
       qtbuy:3,
       unit_price:550_000,
    };
        println!("\nBrand name is {} \n Quantity in stock is {} \n Quantity to buy is {}
        Price per one unit is {} \n Line total is {} ", laptop_3.lpbrand,laptop_3.qtstock,
        laptop_3.qtbuy,laptop_3.unit_price,laptop_3.unit_cost());
   
    let laptop_4 = Laptops{
       lpbrand:String::from("Dell"),
       qtstock:4,
       qtbuy:3,
       unit_price:850_000,
    }; 
        println!("\nBrand name is {} \n Quantity in stock is {} \n Quantity to buy is {}
        Price per one unit is {} \n Line total is {} ", laptop_4.lpbrand,laptop_4.qtstock,
        laptop_4.qtbuy,laptop_4.unit_price,laptop_4.unit_cost());

    let laptops = vec![laptop_1,laptop_2,laptop_3,laptop_4];
    let mut total_cost = 0;

    for laptop in laptops{ // I wasn't sure how to use the impl to do this so i just used a loop
        total_cost += laptop.unit_cost();
    }  
    println!("Total cost of everything for the customer is {} ", total_cost);        

}

