use actix_web::{server, App, HttpRequest, Responder};
use std::env;


fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

// /// Declare a handler.
// #[get("/yell")]
// fn loud_index() -> &'static str {
//     "HELLO, WORLD!"
// }

// /// Declare a handler.
// #[get("/rand/<max>")]
// fn random(max : u8) -> String {
//     let random_number = rand::thread_rng().gen_range(1..max);
//     return format!("Hello for the {}'th time" + random_number);
// }


fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
    App::new()
        .resource("/", |r| r.f(greet))
        .resource("/{name}", |r| r.f(greet))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run();
}