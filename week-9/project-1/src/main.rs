// use std::io;
// use std::io::Write;
// use std::io::Read;
fn main() {
 let mut _file = std::fs::File::create("Nigerian-Brewery-Li.txt").expect("create failed");
 let larger = vec!["33 export", "desperados", "goldberg" , "gulder", "heineiken", "star"];
 let stout = vec!["legend", "turbo king", "williams" ];
 let non_alcoholic = vec!["maltina", "amstel malta", "malta gold", "fayrouz"];

 // println!("categories of the drink");
 // file.write_all("\n Portfolio of high quality lager".as_bytes()).expect("write failed");

// file.write_all("\n LARGER".as_bytes()) .expect("write failed");
// file.write_all("\n 33 export".as_bytes()).expect("write failed");
// file.write_all("\n goldberg".as_bytes()).expect("write failed");
// file.write_all("\n gulder".as_bytes()).expect("write failed");
// file.write_all("\n heineiken".as_bytes()).expect("write failed");
// file.write_all("\n Star".as_bytes()).expect("write failed");


// file.write_all("\n STOUT".as_bytes()).expect("write failed");
// file.write_all("\n legend".as_bytes()).expect("write failed");
// file.write_all("\n turbo king".as_bytes()).expect("write failed");
// file.write_all("\n williams".as_bytes()).expect("write failed");


// file.write_all("\n NON ALCOHOLIC".as_bytes()).expect("write failed");
// file.write_all("\n maltina".as_bytes()).expect("write failed");
// file.write_all("\n Amstel malta".as_bytes()).expect("write failed");
// file.write_all("\n fayrouz".as_bytes()).expect("write failed");
// file.write_all("\n malta gold".as_bytes()).expect("write failed");





// file.write_all(format!(larger,"{:?}")).expect("write failed");



// file.write_all("\nThis is the appendage to the document."
// .as_bytes ()).expect("write failed");


 // for i in 0..larger.len(){
 //    print!("{} is under larger ",larger[i]);
 // }


//  let mut input1 = String::new();
//  io::stdin()
// .read_line(&mut input1)
// .expect("not a valid string");
// let profession

// let mut input1  :Vec<string> = Vec::new()
// io::stdin()
// .read_line(&mut input1)
// .expect("not a valid string");
// let name  = input1 
// .trim()
// .parse()
// .expect("not a valid input");
//  name.push(developers_name);

// for i in 0..larger{
//    print!("{}  ||  {}  ||  {}  ",larger[i], stout[i],non_alcoholic[i] );
// }
// for i in 0..larger{
// println!("Beers under larger 
//           \n {}",larger[i])   
// }

// println!("{}",larger);
// println!("{}",stout);
// println!("{}",non_alcoholic);

// for i in 0..larger.len(){
//    println!("this under larger -> {} ",larger[i]);
// }
// for i in 0..stout.len(){
//    println!("\n this under stout -> {} ",stout[i]);
// }
// for i in 0..non_alcoholic.len(){
//    println!("\n this under non_alcoholic -> {} ",non_alcoholic[i]);
// }

file.write_all(larger.join("\n").as_bytes()).expect("write failed");
file.write_all("\n larger: \n ".as_bytes()). expect("not a valid string");


file.write_all(stout.join("\n").as_bytes()).expect("write failed");
file.write_all("\n stout: \n ".as_bytes()). expect("not a valid string");


file.write_all(non_alcoholic.join("\n").as_bytes()).expect("write failed");
file.write_all("\n non_alcoholic: \n ".as_bytes()). expect("not a valid string");
println!("data succesfully stored");
}

