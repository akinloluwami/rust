pub fn types(){

    //Default i32
    let num = 69;

    //Default f64
    let fl = 6.9;

    //Explicit type
    let exp:i128 = 2345678987654325678909876;

    //Find max size
    println!("i32:{} | i64:{} | i128:{}", std::i32::MAX, std::i64::MAX, std::i128::MAX);
}