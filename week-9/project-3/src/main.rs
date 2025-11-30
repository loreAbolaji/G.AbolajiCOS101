// use std::io::Write;
// fn main() {

    // let names_of_commissioner = vec!["Aigbogbuun " , "Murtala Afeez Bendu", "Okorocha ", "Adewale Jimoh Akanbi", "Oszuwa Faith Etieye"];
    // let ministry = vec!["internal affairs","justice", "defense" , "power&steel" , "petroleum" ];
    // let geopolitical = vec!["south west", "north east" ,"south south", "south west ", "south east"];
    // let efcc = vec![&names_of_commissioner[0],&ministry[0],&geopolitical[0],
    //                 &names_of_commissioner[1],&ministry[1],&geopolitical[1],
    //                 &names_of_commissioner[2],&ministry[2],&geopolitical[2],
    //                 &names_of_commissioner[3],&ministry[3],&geopolitical[3],
    //                 &names_of_commissioner[4],&ministry[4],&geopolitical[4],];


    // let mut count = 0; 

    // for names in names_of_commissioner.iter(){
    //     count+=1;s
    //     println!("Name of commissioner{} \n {}  ",count ,names); }

    // for min in ministry.iter(){
    //     println!("Ministry:\n{}",min); }

    // for geo in geopolitical.iter(){
    //     println!("Zones:\n{}",geo); }     

   // for i in efcc.iter(){
   //  println!("{} | {} | {} ", names_of_commissioner[i], ministry[i] ,geopolitical[i])
   // }
   // println!("{:?}",efcc);


// let mut file = std::fs::File::create("LIST OF CONVICTED MINISTERS.txt").expect("not a valid string");

// let cleaned: Vec<&str> = efcc.iter().cloned().collect();
// file.write_all(cleaned.join("\n").as_bytes()).expect("not a valid string");
// file.write_all(efcc.iter().copied().collect::<Vec<&str>>().join("\n").as_bytes())
//     .expect("not a valid string");
// let mut file = std::fs::File::create("LIST OF CONVICTED MINISTERS.txt")
//     .expect("not a valid string");

// let cleaned: Vec<&str> = efcc.iter().map(|s| *s).collect();
// file.write_all(cleaned.join("\n").as_bytes()).expect("not a valid string");

//error before gettingit right 

use std::io::Write;
fn main() {
    let names_of_commissioner = vec![
        "Aigbogbuun",
        "Murtala Afeez Bendu",
        "Okorocha",
        "Adewale Jimoh Akanbi",
        "Oszuwa Faith Etieye"
    ];

    let ministry = vec![
        "internal affairs",
        "justice",
        "defense",
        "power&steel",
        "petroleum"
    ];

    let geopolitical = vec![
        "south west",
        "north east",
        "south south",
        "south west",
        "south east"
    ];

    let mut file = std::fs::File::create("LIST OF CONVICTED MINISTERS.txt")
        .expect("Failed to create file");

    for i in 0..names_of_commissioner.len() {
        let line = format!(
            "{} | {} | {}\n",
            names_of_commissioner[i],
            ministry[i],
            geopolitical[i]
        );
        file.write_all(line.as_bytes()).expect("Failed to write to file");
    }
}

    
