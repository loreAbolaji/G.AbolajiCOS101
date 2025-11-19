fn main (){
    let datatype_tuple: (&str ,f32 , u8) = ("rust ",3.14 ,100);
    println!("tuple contents = {:?}", tuple);

    let no_datatype_tuple = ("rust ", "fun " ,100);
    println!("tuple contents ={:?}" , tuple);
    println!("valueo of index 0 ={}",datatype_tuple.0);
    println!("valueo of index 1 ={}",datatype_tuple.1);
    println!("valueo of index 2 ={}",datatype_tuple.2);


}