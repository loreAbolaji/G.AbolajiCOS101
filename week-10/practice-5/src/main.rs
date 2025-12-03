fn main() {
    let x = vec![100, 200, 300];
    borrow_vector(&x);
    println!("printing the valur from the main () x[0] = {} " , x[0]);
    println!("*******************************");
}
fn borrow_vector(v:&Vec<i32>){
    println!("*************************");
    println!("inside print vector function {:?} \n ", z);
    println!("-------------------------");
}