fn main() {
    let arr:[i8;4] = [10,20,30,40];
    println!("arry is{:?}", arr);
    println!("array size is :{}",arr.len());

    for val in arr.iter(){
        println!("value is :{}", val);
    }
}
