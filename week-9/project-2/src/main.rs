// use std::io;
use std::io::Write;

fn main (){
    let students = vec![
        ("oluchi mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu" , "ECO10110101" , "Economics" , 100),
        ("Shania Bolade " , "CSC10328828", "Computer" , 200),
        ("Adekunle Gold "  ,"EEE11020202", "Eletrical" ,200),
        ("Blanca Edemoh ", "MEE10202001", "Mechanical", 100),];


        for students in &students{
            println!("\n {} | {} | {} | {} ", students.0, students.1 , students.2 , students.3);
        }

        let mut file = std::fs::File::create("PAU SIMS.txt").expect("not a valid string");
      file.write_all("PAU students Management information system".as_bytes()).expect("not a valid string");
      file.write_all("student name | Matric number  | Department | level".as_bytes()).expect("not a valid string");

}