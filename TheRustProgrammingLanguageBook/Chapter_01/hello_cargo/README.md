
* Cargo.toml
     -[package] information about the package itself 
     -[dependencies] any dependencies or crates that the project will depend on 


* Building a Cargo project 
    ```
    cargo build
    ```
    * Executable will be created in /target/debug/hello_cargo

* Cago.lock 
    - keeps track of dependencies in the application

* To comple and then run 
    ```
    cargo run
    ```
    - if the build was fast or did not build completely that means the the files haven't been changed since the last build