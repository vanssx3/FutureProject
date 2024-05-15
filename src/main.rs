use std::fs::File;
use std::io;
use std::fs;

fn main() {
    //creates password file (insecure)
    File::create("passwords_unsecure.txt")
        .expect("Unable to create :(");

    println!("Password storage created. Enter your password: ");

    //gets user password
    let mut passwordi = String::new();
    io::stdin()
        .read_line(&mut passwordi)
        .expect("Failed to read");

    //writes password to unsecured text file
    fs::write("passwords_unsecure.txt", passwordi)
        .expect("Writing failed - unable to write password");

    println!("Password logged to text file! Cat the file to view stored password.");
    println!("This is an example of insecure password handling - Encryption should be used!")
}