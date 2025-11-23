use std::io;

fn main() {
    // Vector to store (name, years_of_experience)
    let mut candidates: Vec<(String, i32)> = Vec::new();
    let mut num = String::new();

    println!("How many candidates are being interviewed?");
    io::stdin().read_line(&mut num).expect("Invalid input");
    let total: usize = num.trim().parse().expect("Enter a number");

    // Collect candidate data
    for _ in 0..total {
        let mut name = String::new();
        let mut years = String::new();

        println!("Enter candidate name:");
        io::stdin().read_line(&mut name).expect("Invalid input");

        println!("Enter years of programming experience:");
        io::stdin().read_line(&mut years).expect("Invalid input");

        let exp: i32 = years.trim().pars…