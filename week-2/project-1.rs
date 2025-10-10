fn main(){
	println!("program to calculate the compound interest after 5 years");
	
	let p: f64 = 250000.0;
	let r: f64 = 10.0;
	let t: f64 = 5.0;

	//compound interest
	let a = p * (1.0 + (r / 100.0))*t ;

	println!("amount is {} ",a);

	let ci = a - p;

	println!("the compound interest is {}", ci);


}