use std::io;
 
 fn main (){
    let v = vec!['r', 'u', 's', 't', 'a', 'c', 't', 'i', 'a' ,'n'];
    let mut input1 = String::new();
    println!("\n enter an index value btw(0-8)");

    std::io()
    .read_line(&mut input1 )
    .expect("failed to read input");
    let index:usize = input1 
    .trim()
    .parse()
    .expect("not a valid input");

    let ch: Option<&char> = v.get(index);
    value(ch);
 }