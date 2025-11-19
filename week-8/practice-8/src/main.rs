fn  main (){
    let mut mountain_heights = ("everest" ,8848 , "fishtail" , 6993);
    println!(
        "original tuple = {:?}", mountain_heights
    );
    mountain_heights.2 = "lhotse";
    mountain_heights.3 = 8516;
    println!("changed tyuple = {:?}", mountain_heights);
}