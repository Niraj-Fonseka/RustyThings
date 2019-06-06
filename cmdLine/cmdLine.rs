use std::process::Command;




fn main(){

let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
} else {
    Command::new("sh")
            .arg("-c")
            .arg("echo helloss")
            .output()
            .expect("failed to execute process")
};

let hello = output.stdout;

let s = String::from_utf8(hello).expect("Found invalid UTF-8");

println!("Out : {}" , s)


}
