fn main() {
    let v - vec![10,20,30];
    let v2 = v.clone();
    display(v2);
    println!("in main {:?}", v2)// this line has an error
}
fn display(v:Vec<i32>)-> Vec<i32>{ // also here
    println!("inside display {:?}", v)
}