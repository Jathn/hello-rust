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
 * # test_int_type
 * 
 * *Function that tests the Int custom type.*
 * 
 */
fn test_int_type() {
    let int1: customtypes::Int = customtypes::Int::init(10);
    println!("int1 = {} of type: {}", int1.get_value(), int1._get_type());
    let int2: customtypes::Int = customtypes::Int::init(50000);
    println!("int2 = {} of type: {}", int2.get_value(), int2._get_type());
    let int3: customtypes::Int = int1 + int2;
    println!("int1 + int2 = {} of type: {}", int3.get_value(), int3._get_type());
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
    test_int_type();
}
