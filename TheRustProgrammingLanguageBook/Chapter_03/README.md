## Common programming concepts

* Default variables are immutable

* There are trade-offs
    -   Mutating an instane in place may be faster than copying returning newly allocated instances 
    -   With smaller data strutures creating new instances writing in a more functional programming style may be easier to reason about. 

* mut isnt allowed to be used with constants. 
    - Declare constants using the keyword `const` instead of the `let` keyword
    - the type of the value must be annotated 
        ```
        const VARIABLE: u32 = 100_000;

* Shadowing 
    - declaring variables as the same name as the previous variable 
        ```
        fn main(){
            let x = 5;

            let x = x * 2;

            println!("value of x : {}",x);
        }
        ```


* Data types 
    - Data types are pretty similar to what it is a normal programming language 

    - Compound Types 
        - tuple
        ```
        fn main(){
            let tup = (500,6.4,1);
            let (x,y,z) = tup; //destructuring
            println!("The value y is : {}" , y);
        }

        * The tuples also can be accessed by `.` and the index of the value 

    - Arrays once decalready cant grow or shrink in size
    - Allocated in the stack

* Functions uses snake case

* function parameters are written 
    ```
    fn function_name(x: i32) {
        println!("The value for x is : {}",x);
    }

* Adding a semicologo into an expression makes it a statement

* Rust will not try to convert non-booean types to a boolean