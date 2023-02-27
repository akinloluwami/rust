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
    // println!("Contains 'a': {}", text.contains("a"));


    //Replace
    // println!("{}", text.replace("i", "a"));


    //Loop through strig by whitespace
    let voice = "Even if you're not ready for the day, it cannot always be night";
    for word in voice.split_whitespace(){
        // println!("{}",word);
    }

    //Split by character
    let url = "https://marcchen.hashnode.dev/building-desktop-applications-with-tauri-nextjs-firebase";
    let parts: Vec<&str> = url.split("/").collect();
    let path : Vec<&str> =  parts[3].split("-").collect(); 
    println!("{:?}", path);

}