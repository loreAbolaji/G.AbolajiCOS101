use std::io;

fn main() {
    println!("Rust program to that displayes the following menu for food items and takin g order");


   println!("welcome to food canteen"); 
   println!(" \n#code  ||     menu                     || price");
   println!(" \np      ||  poudo yam/ edinkainko soup  || 3200");
   println!(" \nf      ||  fried rice and chicken      || 3000");
   println!(" \na      ||  poudo yam/ edinkainko soup  || 2500");
   println!(" \ne      ||  eba and egusi soup          || 2000");
   println!(" \nW      ||  white rice and stew         || 2500") ;

   println!("\n choose your choice of food code as displayed above");
   let mut firstpick = String::new();
   io::stdin()
   .read_line(&mut firstpick)
   .expect("not a valid string");
   let code = firstpick.trim().to_lowercase();

   println!("\n How many?");
   let mut input2 = String::new();
   io::stdin()
   .read_line(&mut input2)
   .expect("not a valid string");
   let quantity :f32 = input2
   .trim()
   .parse()
   .expect("not a valid string");

   let price = match code.as_str(){

    "p" => 3200.0,
    "f" => 3000.0,
    "a" => 2500.0,
    "e" => 2000.0,
    "w" => 2500.0,
    _=> {
        println!("please input a valid item code");
        return;
    }
   };

   let total = price * quantity;
   let discount = total - (0.5 * total);
   if total >10000.0 {
    println!("your total is passed 10000 you have been gievn a discount of 0.5% here is your balance {}", discount);
   }

    else if total <=10000.0 {
    println!("your total is  {}", total);
   }

// println!("do you wish to continue ( y/n )");
// let mut input3 = String::new();
// io::stdin()
// .read_line(&mut input3)
// .expect("not a valid string");


}
