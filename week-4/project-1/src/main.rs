use std::io;
fn main(){

    let mut input1 = String::new();
    // let mut input2 = String::new();
    let mut input3 = String::new();


    println!("\n enter your full name");
    io::stdin()
    // .parse()
    .read_line(&mut input1)
    .expect("not a valid string");


    // println!("\n Are you experienced (yes/no)");
    // io::stdin()
    // // .parse()
    // .read_line(&mut input2)
    // .expect("it must be either yes or no");
    // let experience = input2;
    
    // // .expect("it must be either yes or no");


    // if experience =="yes" {
    //     println!("you are experienced {}",input1);
    // }else{
    //     println!("you are inexperienced {}",input1);
    // }

    // if experience =="no"{
    //     println!("you are inexperienced {}",input1);
    // }else{
    //     println!("you are experienced {}",input1);
    // }

    println!("\n enter your age");
    io::stdin()
    // .parse()
    .read_line(&mut input3)
    .expect("not a valid string");
    let age:i32 = input3
    .trim()
    .parse()
    .expect("not a valid number ");
    
    
    if age>= 40{
        println!("you have the adequate experience {} your incentive is 1_560_000 {}", input1,input1);}


    else if age >=30 && age <40 {
        println!(" you are experienced {} your incentive is 1_480_000 {}.",input1,input1);}

        else if age <20  {
        println!("you are inexperienced {} your incentive is 100_000 {}.",input1,input1);} 


    else if age <30 {
        println!("you are experienced {} your incentive is 1_300_000 {}.",input1,input1);}  

     
    

    


}