struct employee{
    name :String,
    company:String,
    age :u32
}
fn main() {
    let emp1 = employee{
        company:String::from("enrst & young"),
        name:String::from("ebiiong jessica"),
        age :25
    };
    println!("name = {} \n ", emp1.name);
    println!("company= {} \n ", emp1.company);
    println!("age = {}", emp1.age);
}
