#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::Data;
use std::io;

#[get("/")]
fn index() -> &'static str {
    "4"
}

#[post("/", data = "<data>")]
fn index_post(mut data: Data) -> io::Result<String> {
    data.stream_to(&mut io::stdout())
        .map(|n| format!("Wrote {} bytes.\n", n))        
}


fn main() {
    rocket::ignite().mount("/", routes![index, index_post]).launch();
}
