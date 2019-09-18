
extern crate rand; //letting tust know that we are using an externam dependency

use std::io; //use the standard library 
use rand::Rng; 
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");


    //thread_rng() function gives a random number generator thats lcoal to the current thread
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    
    println!("The secret number is : {}",secret_number);
    
    println!("Please input your guess");

    // let = immutable 
    // let mut = mutable

    // String::new()
    // means new function is called an associated function with String type 
    // this is a static methond because its being directly called on the type name not an instance of that type
    let mut guess = String::new(); 

    //stdin() is also an associated function
    //the guess variable should also be mutable thats why its passed in by reference
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); //expect handles the Result type that gets returned from read_line()


    //shadowing the previous guess with a new guess
    //.trim() removes the /n character from the end of the string
    //parse() function parses a string into a number type.
    //The type is defined at the declaration of the variable 

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number !");


    println!("You guessed : {}" , guess); //{} holds a value in place

    //match is an expressing with arms
    //Its almost like an else if 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
