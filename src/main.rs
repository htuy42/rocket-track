//! A simple Rocket application, based on the example in [Getting Started][].
//!
//! [Getting Started]: https://rocket.rs/v0.4/guide/getting-started/

#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes};
use rand::Rng;


/// Declare a handler.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Declare a handler.
#[get("/yell")]
fn loudIndex() -> &'static str {
    "HELLO, WORLD!"
}

/// Declare a handler.
#[get("/rand/<max>")]
fn rand(max : i64) -> String {
    let random_number = rand::thread_rng().gen_range(1..max);
    format!("Hello for the {}'th time" + random_number);
}


/// Start our server.
fn main() {
    rocket::ignite().mount("/", routes![index, loudIndex, rand]).launch();
}
