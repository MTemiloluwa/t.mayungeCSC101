fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Calculate Compound Interest

	let a:f64 = p * (1.0 - (r / 100.00)).powf(n);
	println!("Amount is {}", a);
	let ci:f64 = p - a;
	println!("Compound interest is {}", ci);
}