## Chapter 2 

* let = makes a variable immutable 
* let mut = makes a variable mutable

* When chaining methods its always good to break up long lines. 
* Two lines for two method calls 
    ```
    foo().bar()
        .foobar();
    ```

* Rust has number of Result types 
* These are enumerations ( enums )
    - They can hae fixed set of values / varients 
* Result types are used to pack errors into it 

* Cargo understands semantic versioning when importing dependencies

* Cargo.lock file ensures reproducible builds 
    - when the application builds in the future cargo will see that the cargo.lock file exists and then use the version specified there instead of pulling the depdnency all over again

* to update your dependencies for the latest version
    ```
    cargo update 
    ```

* to get dependency documentation 
    ```
    cargo doc --open
    ```
    - will open up documentation provided by all the dependncies locally and open in the browser

* Rust has strong and static type system.
* It also has type inference 
    ```
    let guess = String::new()
    ```
    - makes guess a string 

* Type shadowing is usually used when you want to convert a variable from one type to another
