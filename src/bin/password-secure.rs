use std::fs::File;
use std::io;
use std::fs;
use sha256::digest;

fn main() {
    //creates password file (insecure)
    File::create("passwords_secure.txt")
        .expect("Unable to create :(");

    println!("Password storage created. Enter your password: ");

    //gets user password
    let mut passwordi = String::new();
    io::stdin()
        .read_line(&mut passwordi)
        .expect("Failed to read");

    //sha256 encryption
    let val = digest(passwordi.clone());
    let password_s = val;
    println!("{}", password_s);

    //writes password to unsecured text file
    fs::write("passwords_secure.txt", password_s)
        .expect("Writing failed - unable to write password");

    println!("Password logged to text file! Cat the file to view stored password.");
    println!("This is an example of secure password handling - Encryption prevents hacking!")
}