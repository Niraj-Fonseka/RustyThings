
fn main() {

    /*
    let x = 5;

    println!("The value of x is : {}" ,x);

    x = 6;

    ^^ Above doesn't work because x is immutable
    */
    let mut x = 6;

    println!("The value of x is : {}",x);

    x = 10;

    println!("The value of x is : {}",x);
    
}
