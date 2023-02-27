pub fn strings(){

    //Immutable fixed length string
    let text = "Hii";

    //Growable, heap-memory allocated
    let mut msg = String::from("Hello ");
    // println!("{}", msg);

    //Get length
    // println!("{}", msg.len());

    //Push single character
    msg.push('W');
    
    //Push multiple characters
    msg.push_str("orld");

    // println!("{}", msg);
    
    //Is empty
    // println!("Is empty: {}", text.is_empty());

    //Contain
    println!("Contains 'a': {}", text.contains("a"));


    //Replace
    println!("{}", text.replace("i", "a"))

}