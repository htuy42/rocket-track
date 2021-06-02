#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes};
use rand::Rng;
use std::cmp::Ordering;

/// Declare a handler.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Declare a handler.
#[get("/yell")]
fn loud_index() -> &'static str {
    "HELLO, WORLD!"
}

/// Declare a handler.
#[get("/random_response/<max>")]
fn random_response(max: u8) -> String {
    let random_number = rand::thread_rng().gen_range(1..max);
    return format!("Hello for the {}'th time", random_number);
}

/// Declare a handler.
#[get("/random_no_max")]
fn random_response_no_max() -> String {
    let random_number = rand::thread_rng().gen_range(1..100);
    return format!("Hello for the {}'th time", random_number);
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/guess/max/guess")]
fn guess(max: u8, guess: u8) -> String {
    let random_number = rand::thread_rng().gen_range(1..max);
    match random_number.cmp(guess) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

/// Start our server.
fn main() {
    rocket::ignite().mount("/", routes![random_response, random_response_no_max, hello, guess]).launch();
}
