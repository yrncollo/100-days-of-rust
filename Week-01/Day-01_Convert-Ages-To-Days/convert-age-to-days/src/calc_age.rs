use std::io;

pub fn calculate_age() {
    
    println!("Enter your age: ");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("failed to read line");
    
    let age:u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a real number")
        
    };

    let results = age * 365;

    println!("Your age ({}yrs) converted to days is {}", age, results)

}
