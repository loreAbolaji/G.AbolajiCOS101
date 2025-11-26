// use std::io;
use std::io::Write;
use std::io::Read;
fn main() {
   let mut file = std::fs::File::create("Nigerian-Brewery-Li.txt").expect("create failed");
 let larger = vec!["33 export", "desperados", "goldberg" , "gulder", "heineiken", "star"];
 let stout = vec!["legend", "turbo king", "williams" ];
 let non_alcoholic = vec!["maltina", "amstel malta", "malta gold", "fayrouz"];

 // println!("categories of the drink");
 file.write_all("\n Portfolio of high quality lager".as_bytes()).expect("write failed");

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





file.write_all(writeln!(larger,"{:?}")).expect("write failed");



// file.write_all("\nThis is the appendage to the document."
// .as_bytes ()).expect("write failed");


 // for i in 0..larger.len(){
 //    print!("{} is under larger ",larger[i]);
 // }
}
