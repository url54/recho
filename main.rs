//! # Recho
//!
//! `recho` is a small user friendly command-line tool for mimicking the Unix style echo
//! command. It has the same built-in functionality of reprinting with-or-without a newline
//! character. It also has the functionality to encode/decode base64, from strings-to strings.
//! This was added because unlike in Unix environments where there is a handy command-line tool to
//! pipe your echo command results to (base64), Windows requires more setup and functionality
//! to perform this operation. Now users on Windows can run `recho` with the same ease
//! as Unix style Echo | base64.

use std::env;
/// Allow unused_imports as not all users will be base64 encoding/decoding.
#[allow(unused_imports)]
use base64::{encode, decode};

fn main() {
    let args: Vec<String> = env::args().collect();
    command_directory(&args);
}

/// The main echo function, uses boolean logic to determine if a newline character is needed.
/// # Examples
/// ```
/// recho.exe -n "Hello world"
/// recho.exe "Hello world"
/// ```
fn base_echo(x: &str, y: bool) {
    let _parameters = x;
    let ending = if y { "\n" } else { "" };
    println!("{}{}", _parameters, ending)
}

/// The main base64-echo logic. Uses boolean logic to determine if data is already base64 encoded
/// or not.
/// # Examples
/// ```
/// recho.exe -tb "Hello world"
/// recho.exe -fb "SGVsbG8gd29ybGQ="
/// ```
/// The first example will encode "Hello world" to base64
/// The second example will decode "SGVsbG8gd29ybGQ=" to "Hello world"
fn echo_base64(x: &str, y: bool) {
    let _parameters = x;
    if y {
        let data = String::from_utf8(decode(_parameters).unwrap()).unwrap();
        print!("{}", data);
    } else {
        print!("{}", encode(_parameters));
    }
}

/// This is the 'switchboard', that handles the arguments that are passed
/// to the program. Since there isn't a lot of functionality built into Echo, this
/// doesn't require a package to handle, like clap.
fn command_directory(args: &[String]) {
    match args.len() {
        2 => if args[1] == "-h" && args.len() == 2 {
            help_directory();
        } else {
            base_echo(&args[1], true);
        },
        3 => if args[1] == "-n" {
            base_echo(&args[2], false);
        } else if args[1] == "-tb" {
            echo_base64(&args[2], false);
        } else if args[1] == "-fb" {
            echo_base64( &args[2], true);
        } else {
            error_directory();
        },
        _ => error_directory(),
    }
}

/// Since not using Clap, or other argument parsers. This is used to generate the typical
/// command-line help information.
fn help_directory() {
    println!("recho: GNU ECHO re-written in Rust with added functionality");
    println!("Usage and Examples:");
    println!("Switches:");
    println!("    -h : help, displays this");
    println!("    -n : no new line character added at the end");
    println!("    -tb : to-base64, encodes the string in base64, automatically passes the (n) switch as well.");
    println!("    -fb : from-base64, decodes the string from base64, automatically passes the (n) switch as well.");
    println!("Usage:");
    println!("    NOTE: Strings containing spaces need to be wrapped in quotes.");
    println!("    recho.exe <SWITCHES> <TEXT>");
    println!("    recho.exe -n 'hello world'");
    println!("    recho.exe 'hello world'");
    println!("    recho.exe -tb 'hello world' (Will print out aGVsbG8gd29ybGQ=)");
}

/// Added a function for the different invalid arguments/parameters that can occur
/// instead of adding them to each different arm-branch, in which they could occur.
fn error_directory() {
    eprintln!("Invalid arguments/parameters\nUsage: recho <SWITCHES> <TEXT>\nExample: recho -n HELLO\nExample: recho -n 'Hello world'");
    eprintln!("Use recho.exe -h, for more information.");
}