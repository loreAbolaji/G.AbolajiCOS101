use std::io;


fn main (){
    let v = vec!['c','o', 'm', 'p', 'u', 't', 'e' ,'r'];
    let mut input = String::new();
    println!("enter tan index value btw (0-7)");
    io::stdin()
    .read_line(&mut input1)
    .expect("failed to read input  ");


    let index:usize = input1.trim().parse().expect("ivalid input");

    let ch : char = v[index];
    println!("{} is the character for index [{}] \n",ch, index);
}