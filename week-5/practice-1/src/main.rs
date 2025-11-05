fn main() {
    let name = "Aisha lawal";
    let  uni:&str = "pan atlantic University";
    let addr:&str = "Km 52 Lekki Epe Expressway, ibeju lekki , lagos";
    println!("name: {}",name );
    println!("university :{}, \n address: {}", uni ,addr);



    let department :&'static str = "computer science";
    let school: &'static str = "school of science and technology";

    println!("department :{}, \n school:{}", department , school);
}
