#![feature(proc_macro_hygiene, decl_macro)]


use rocket::{self, get, routes};
use rocket::State;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Mutex;

#[get("/less/<amount>")]
fn decrease(current: State<Mutex<i32>>, amount: i32) -> String {
    *current.lock().unwrap() -= amount;
    format!("Count is now {}, down by {}", current.lock().unwrap(), amount)
}

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

#[get("/guess/<max>/<guess>")]
fn guess(max: u8, guess: u8) -> &'static str {
    let random_number = rand::thread_rng().gen_range(1..max);
    match &guess.cmp(&random_number) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

/// Start our server.
fn main() {
    rocket::ignite()
        .mount("/", routes![loud_index, random_response, random_response_no_max, hello, guess, decrease])
        .manage(Mutex::new(5))
        .launch();
}
