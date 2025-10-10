fn main(){
	
	println!("program to calculate the sum and average of the following sales record");


	let a: f64 = 450000.00;
	let b: f64 = 1500000.00;
	let c: f64 = 750000.00;
	let d: f64 = 2850000.00;
	let e: f64 = 250000.00;

	//sum

	let sum = a + b + c + d + e;

	println!("the sum of of the sales record is {}" ,sum);

	let average = (a + b + c + d + e) / 5.0;

	println!("the average of the sales record is {}" ,average);


}