use std::io;

fn main() {


    let mut input1 =  String::new();
    let mut input2 =  String::new();

    println!("Enter name:");
    io::stdin()
    .read_line(&mut input1)
    .expect("not a valid string");

    println!("Enter age:");
    io::stdin()
    .read_line(&mut input2)
    .expect("not a valid string");
    let age :i32 = input2
    .trim()
    .parse()
    .expect("not a valid number ");


    if age >=18{
        println!("welcome ro the party {}", input1);
    }


        else{
             println!("oops , youre not of age to enter the party:{}",input1);
        }
       
    
}
