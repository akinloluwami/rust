pub fn vars() {
    
    //Immutable by default
    let  name = "Akinkunmi";

    //Mutable
    let mut age =  20;

    age = age + 1;

    println!("{} is {} years old", name, age);

    //Define constant
    const ID: i32 = 001;

    println!("{}", ID);

    //Assign multiple vars
     let (name_of_building, feet) = ("Burj Khalifa", 2720);

     println!("The {} is {} feet tall", name_of_building, feet);
}