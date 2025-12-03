struct employee{
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    let emp1 = employee{
        company:String::from("microsoft coporation"),
        ceo:String::from("satya nadela"),
        age : 56

    };
    let emp2 = employee{
        company:String::from("google inc"),
        ceo:String::from("sundai pinchai"),
        age : 51

    };

    display(emp1);
    display(emp2);

}
fn display(emp:employee){
    println!("name is {} company is {} age is {}", emp.ceo , company.ceo , emp.age);
}
