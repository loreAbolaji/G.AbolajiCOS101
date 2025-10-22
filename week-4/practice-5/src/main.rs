use std::io;

fn main() {


    let mut input =  String::new();

   

    println!("\n ente your height (in cetimeteres):");
    io::stdin()
    .read_line(&mut input)
    .expect("not a valid string");
    let height :f32 = input
    .trim()
    .parse()
    .expect("not a valid number ");


    if height >= 150.0 && height <= 170.0{
        println!("you are an average person");
    }
    else if height > 170.0 && height <= 195.0{
        println!("you are tall");
     }
     else if height  > 170.0 && height >100.0{
        println!(" youre a dwarf");
     }
     else  {
        println!("Abnormal height");
     } 
    
}
