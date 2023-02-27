pub fn strings(){

    //Immutable fixed length string
    let _text = "Hi";

    //Growable, heap-memory allocated
    let msg = String::from("Even if you're not ready for the day, it cannot always be night.");

    println!("{}", msg)
}