use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("\n enter your full name");
    io::stdin()
    // .parse()
    .read_line(&mut input1)
    .expect("not a valid string");
   

    println!("\n a?");
    io::stdin()
    .read_line(&mut input2)
    .expect("not a valid String");
    let a: f32 = input2
    .trim()
    .parse()
    .expect("not a valid number");

        println!("\n b?");
    io::stdin()
    .read_line(&mut input3)
    .expect("not a valid String");
    let b: f32 = input3
    .trim()
    .parse()
    .expect("not a valid number");

        println!("\n c?");
    io::stdin()
    .read_line(&mut input4)
    .expect("not a valid String");
    let c: f32 = input4
    .trim()
    .parse()
    .expect("not a valid number");

    
    let d  = b.powf(2.0) - 4.0 * a * c;
    println!(" your discriminant {} is {}",input1 , d);


    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots");
    }
    
} 