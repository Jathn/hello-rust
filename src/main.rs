/**
 * File: main.rs
 */
mod customtypes;
/**
 * # hello_rust
 * 
 * *Function that returns a greeting message.*
 * 
 * ## Parameters
 * 
 * - `Author`: A string slice (`&str`) that represents the name of the author.
 * 
 * ## Returns
 * 
 * A `String` containing a a greeting, with a specification who sent it.
 * 
 * ## Example
 * 
 * HelloRust("Dan") would return the following:
 * 
 * Hello Rust.
 * ~ Dan
 */
fn hello_rust(author: &str) -> String {
    let result: String = format!("Hello Rust.\n~ {}", author);
    return result;
}

/**
 * # main
 * 
 * Prints the following string:<br>
 * Hello Rust.<br>
 * ~ Jathn<br>
 * to the command line.
 * 
 */
fn main() {
    let greeting: String = hello_rust("Jathn");
    println!("{}", greeting);
}
