fn main() {
	let t:f64 = 450_000.00 * 2.0;// t for toshiba
	let m:f64 = 1500_000.00 * 1.0;// m for macbook
	let h:f64 = 750_000.00 * 3.0;// h for HP
	let d:f64 = 2850_000.00 * 3.0;// d for dell
	let a:f64 = 250_000.00 * 1.0;// a for Acer


     // Calculate total sum
    let sum:f64 = t + m + h + d + a;
    println!("Total Sum is {}", sum);
    
    // calculate the average
    let count:f64 = 5.0;
    println!("Count is {}", count);
    let average:f64 = sum / count;
    println!("Average is {}", average);




}