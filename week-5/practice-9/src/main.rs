fn main (){

    let a:i32 = 10;
    let b:i32 = 10;
    println!("value of a:{}",a);
    println!("value of b:{}",b);

    let mut res = a >b;
    println!("A greater than B:{}",res);

    res = a <b;
    println!("A greater than B:{}",res);


     res = a >=b;
    println!("A less than B:{}",res);


     res = a <=b;
    println!("A lesser than  or equls to B:{}",res);


     res = a == b;
    println!("A is equals to  B:{}",res);


    res = a !=b;
    println!("A is not equas B:{}",res);



}