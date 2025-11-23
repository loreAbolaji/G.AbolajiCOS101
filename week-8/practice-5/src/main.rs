use std::io;
fn main (){
    let mut city :Vec<String> = Vec::new();
    println!("the city vector has element {} ", city .len());
    let  mut input1 = String::new();
    println!("how many cities do you want to enter ");
    io::stdin()
    .read_line(&mut input1)
    .expect("not a valid input");
    let city_num :i32 = input1
    .trim()
    .parse()
    .expect("not a valid input");
    for count in 0..city_num{
        let mut input2 = String::new();
        println!("Enter city{}", count+1);
        io::stdin()
        .read_line(&mut input2)
        .expect("not a valid input");
        let new_city:String = input2 
        .trim()
        .parse()
        .expect("not a valid input");
        city.push(new_city)

    }
    print!("your preferred citis are :\n");
    let mut count=1;
    for i in city {
        println!("{} {}", count ,i);
        count+=1;
    }
}