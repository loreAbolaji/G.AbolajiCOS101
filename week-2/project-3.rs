fn main(){

	println!("program to find the depreciated value after 3 years");

	let p: f64 =510000.0;
	let r: f64 =5.0;
	let t: f64 =3.0;

	//depreciation

	let a = p *(1.0 - (r / 100.0) ) * t;

	println!("the amount is {}",a);

	let dp = p - a;

	println!("the depreciated value is {}", dp);

}